use crate::state;
use std::io::Write;
use std::process::{Command, Stdio};
use std::path::PathBuf;

pub fn run_show() {
    let state = state::load_state();
    if state.history.is_empty() {
        println!("History is empty.");
        return;
    }

    // Build preview list: newest first.
    // Each line is prefixed with a 1-based position number so that:
    //   a) wofi exibe na ordem correta (sem reordenar alfabeticamente)
    //   b) podemos recuperar o item pelo número mesmo que o wofi
    //      remova os espaços iniciais antes de devolver a seleção.
    //
    // Formato: "1 | preview text" (sem espaços iniciais para não depender do wofi)
    let items: Vec<(usize, String)> = state.history
        .iter()
        .rev()
        .enumerate()
        .map(|(i, text)| {
            let preview: String = text.replace('\n', " ").chars().take(75).collect();
            (i, format!("{} | {}", i + 1, preview))
        })
        .collect();

    let input_text = items
        .iter()
        .map(|(_, line)| line.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    // CSS path installed by justfile
    let css_path: Option<PathBuf> = std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".config/wofi/cosmic-clip.css"));

    let mut wofi_args: Vec<String> = vec![
        "--show".into(),
        "dmenu".into(),
        "--prompt".into(),
        "📋 Clipboard".into(),
        "--insensitive".into(),
        "--no-actions".into(),
    ];

    if let Some(ref css) = css_path.filter(|p| p.exists()) {
        wofi_args.push("--style".into());
        wofi_args.push(css.to_string_lossy().into_owned());
    }

    let launcher = Command::new("wofi")
        .args(&wofi_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .or_else(|_| {
            Command::new("fuzzel")
                .args(["-d", "-p", "Clipboard> "])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
        });

    let Ok(mut child) = launcher else {
        eprintln!("❌ Fuzzy finder não encontrado (wofi ou fuzzel necessário).");
        return;
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(input_text.as_bytes());
    }

    let Ok(output) = child.wait_with_output() else {
        return;
    };

    let selected_line = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected_line.is_empty() {
        return; // usuário cancelou
    }

    eprintln!("[DEBUG] Selecionado: {:?}", &selected_line);

    // Recupera o item pelo índice numérico no prefixo ("N | preview").
    // Isso é robusto contra o wofi remover/alterar espaços: basta parsear
    // o número antes do primeiro " | ".
    let found_text: Option<String> = selected_line
        .split_once(" | ")
        .and_then(|(num_str, _)| num_str.trim().parse::<usize>().ok())
        .and_then(|pos_1based| {
            // pos_1based é 1-indexed → converte para índice em `items`
            let idx = pos_1based.checked_sub(1)?;
            let (rev_idx, _) = items.get(idx)?;
            state.history.iter().rev().nth(*rev_idx).cloned()
        })
        // Fallback: busca pelo conteúdo do preview caso o parse falhe
        .or_else(|| {
            let preview_part = selected_line
                .split_once(" | ")
                .map(|(_, p)| p)
                .unwrap_or(&selected_line);
            state.history.iter().rev().find(|text| {
                let preview: String = text.replace('\n', " ").chars().take(75).collect();
                preview == preview_part
            }).cloned()
        });

    let Some(text) = found_text else {
        eprintln!("❌ Não foi possível encontrar o item no histórico.");
        eprintln!("   Linha retornada pelo wofi: {:?}", selected_line);
        return;
    };

    eprintln!("[DEBUG] Copiando para clipboard: {:?}", &text.chars().take(40).collect::<String>());

    // Escreve no clipboard via wl-copy SEM chamar .wait().
    // No Wayland o clipboard funciona como servidor: o processo que escreveu
    // precisa ficar vivo para responder ao Ctrl+V. Ao não chamar .wait(),
    // o wl-copy continua rodando em background como dono do clipboard até
    // que outra app copie algo novo.
    match Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
        Ok(mut wl_copy) => {
            if let Some(mut stdin) = wl_copy.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
                // fechar o stdin sinaliza EOF → wl-copy sabe que recebeu tudo
            }
            // drop do handle NÃO mata o processo — ele continua servindo Ctrl+V
            println!("✅ Clipboard: {:?}", &text.chars().take(40).collect::<String>());
        }
        Err(e) => {
            eprintln!("❌ wl-copy não encontrado: {e}");
            eprintln!("   Instale com: sudo apt install wl-clipboard");
        }
    }
}

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

    // Format for display: one line per item, newest first
    let input_lines: Vec<String> = state.history.iter().enumerate().rev().map(|(i, text)| {
        let single_line: String = text.replace('\n', " ").chars().take(80).collect();
        format!("{}: {}", i, single_line)
    }).collect();

    let input_text = input_lines.join("\n");

    // CSS path instalado pelo justfile
    let css_path: Option<PathBuf> = std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".config/wofi/cosmic-clip.css"));

    let mut wofi_args: Vec<String> = vec![
        "--show".into(),
        "dmenu".into(),
        "--prompt".into(),
        "📋 Clipboard".into(),
        "--insensitive".into(),
    ];

    if let Some(ref css) = css_path {
        if css.exists() {
            wofi_args.push("--style".into());
            wofi_args.push(css.to_string_lossy().into_owned());
        }
    }

    let child = Command::new("wofi")
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

    if let Ok(mut child) = child {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input_text.as_bytes());
        }

        if let Ok(output) = child.wait_with_output() {
            let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if selected.is_empty() {
                return; // Usuário cancelou
            }

            if let Some(idx_str) = selected.split(':').next() {
                if let Ok(idx) = idx_str.parse::<usize>() {
                    if let Some(text) = state.history.get(idx) {
                        // Usa wl-copy para escrever no clipboard (mais confiável em headless)
                        let mut wl_copy = Command::new("wl-copy")
                            .stdin(Stdio::piped())
                            .spawn()
                            .expect("Falha ao executar wl-copy. Instale: sudo apt install wl-clipboard");

                        if let Some(mut stdin) = wl_copy.stdin.take() {
                            let _ = stdin.write_all(text.as_bytes());
                        }
                        let _ = wl_copy.wait();
                        println!("Clipboard atualizado com: {:?}", &text.chars().take(40).collect::<String>());
                    }
                }
            }
        }
    } else {
        println!("Fuzzy finder not found (wofi or fuzzel needed).");
    }
}

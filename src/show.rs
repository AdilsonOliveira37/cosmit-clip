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
    //   a) wofi displays in the correct order (without reordering alphabetically)
    //   b) we can retrieve the item by number even if wofi
    //      removes the leading spaces before returning the selection.
    //
    // Format: "1 | preview text" (no leading spaces to avoid depending on wofi)
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
        eprintln!("❌ Fuzzy finder not found (wofi or fuzzel required).");
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
        return; // user cancelled
    }

    eprintln!("[DEBUG] Selected: {:?}", &selected_line);

    // Retrieve the item by the numeric index in the prefix ("N | preview").
    // This is robust against wofi removing/altering spaces: just parse
    // the number before the first " | ".
    let found_text: Option<String> = selected_line
        .split_once(" | ")
        .and_then(|(num_str, _)| num_str.trim().parse::<usize>().ok())
        .and_then(|pos_1based| {
            // pos_1based is 1-indexed → convert to index in `items`
            let idx = pos_1based.checked_sub(1)?;
            let (rev_idx, _) = items.get(idx)?;
            state.history.iter().rev().nth(*rev_idx).cloned()
        })
        // Fallback: search by preview content if parsing fails
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
        eprintln!("❌ Could not find the item in the history.");
        eprintln!("   Line returned by wofi: {:?}", selected_line);
        return;
    };

    eprintln!("[DEBUG] Copying to clipboard: {:?}", &text.chars().take(40).collect::<String>());

    // Write to clipboard via wl-copy WITHOUT calling .wait().
    // On Wayland the clipboard acts as a server: the process that wrote
    // needs to stay alive to answer to Ctrl+V. By not calling .wait(),
    // wl-copy keeps running in the background as the clipboard owner until
    // another app copies something new.
    match Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
        Ok(mut wl_copy) => {
            if let Some(mut stdin) = wl_copy.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
                // closing stdin signals EOF → wl-copy knows it received everything
            }
            // dropping the handle DOES NOT kill the process — it keeps serving Ctrl+V
            println!("✅ Clipboard: {:?}", &text.chars().take(40).collect::<String>());
        }
        Err(e) => {
            eprintln!("❌ wl-copy not found: {e}");
            eprintln!("   Install with: sudo apt install wl-clipboard");
        }
    }
}

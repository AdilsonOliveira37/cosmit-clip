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

    // Build preview list: newest first, one line each (max 80 chars)
    let previews: Vec<String> = state.history.iter().rev().map(|text| {
        text.replace('\n', " ").chars().take(80).collect()
    }).collect();

    let input_text = previews.join("\n");

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
    ];

    if let Some(ref css) = css_path.filter(|p| p.exists()) {
        wofi_args.push("--style".into());
        wofi_args.push(css.to_string_lossy().into_owned());
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
            let selected_preview = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if selected_preview.is_empty() {
                return; // user cancelled
            }

            // Find the full text matching the selected preview.
            // Iterates newest-first so duplicates resolve to the most recent copy.
            let found = state.history.iter().rev().find(|text| {
                let preview: String = text.replace('\n', " ").chars().take(80).collect();
                preview == selected_preview
            });

            if let Some(text) = found {
                let mut wl_copy = Command::new("wl-copy")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("wl-copy not found. Install it with: sudo apt install wl-clipboard");

                if let Some(mut stdin) = wl_copy.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                let _ = wl_copy.wait();
                println!("Clipboard updated: {:?}", &text.chars().take(40).collect::<String>());
            }
        }
    } else {
        println!("Fuzzy finder not found (wofi or fuzzel required).");
    }
}

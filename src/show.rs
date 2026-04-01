use crate::state;
use std::io::Write;
use std::process::{Command, Stdio};
use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub fn run_show() {
    let state = state::load_state();
    if state.history.is_empty() {
        println!("History is empty.");
        return;
    }

    // Format for display to avoid breaking dmenu structure on multiline
    let input_lines: Vec<String> = state.history.iter().enumerate().rev().map(|(i, text)| {
        let mut single_line = text.replace('\n', " ");
        single_line.truncate(80); // take first 80 chars
        format!("{}: {}", i, single_line)
    }).collect();
    
    let input_text = input_lines.join("\n");

    let child = Command::new("wofi")
        .args(["--show", "dmenu", "--prompt", "Clipboard"])
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
            if let Some(idx_str) = selected.split(':').next() {
                if let Ok(idx) = idx_str.parse::<usize>() {
                    if let Some(text) = state.history.get(idx) {
                        let opts = Options::new();
                        let _ = opts.copy(
                            Source::Bytes(text.clone().into_bytes().into()),
                            MimeType::Autodetect
                        );
                    }
                }
            }
        }
    } else {
        println!("Fuzzy finder not found (wofi or fuzzel needed).");
    }
}

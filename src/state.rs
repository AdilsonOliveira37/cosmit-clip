use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub history: Vec<String>,
}

pub fn get_state_file() -> PathBuf {
    PathBuf::from("/tmp/cosmic-clip-minimal.json")
}

pub fn load_state() -> State {
    if let Ok(content) = fs::read_to_string(get_state_file()) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        State::default()
    }
}

pub fn save_state(state: &State) {
    if let Ok(content) = serde_json::to_string(state) {
        let _ = fs::write(get_state_file(), content);
    }
}

pub fn push_item(text: String) {
    let mut state = load_state();
    if state.history.last() == Some(&text) {
        return; // Avoid sequential duplicates
    }
    state.history.retain(|x| x != &text); // Remove previous occurrences to move to the end
    state.history.push(text);
    while state.history.len() > 50 {
        state.history.remove(0);
    }
    save_state(&state);
}

use wayland_clipboard_listener::{WlClipboardPasteStream, WlListenType};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};
use std::io::Read;
use crate::state;

pub fn run_daemon() {
    let mut stream = WlClipboardPasteStream::init(WlListenType::ListenOnCopy)
        .expect("Failed to initialize clipboard listener");

    println!("Daemon started. Listening to Wayland clipboard events...");

    for _event in stream.paste_stream() {
        if let Ok((mut pipe, _)) = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text) {
            let mut contents = vec![];
            if pipe.read_to_end(&mut contents).is_ok() {
                 let text = String::from_utf8_lossy(&contents).into_owned();
                 if !text.trim().is_empty() {
                     state::push_item(text);
                 }
            }
        }
    }
}

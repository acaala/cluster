use arboard::Clipboard;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct ClipboardItem {
    pub content: String,
}

impl ClipboardItem {
    pub fn new(content: String) -> ClipboardItem {
        ClipboardItem { content }
    }
}

pub fn store_clipboard_contents(
    clipboard: &mut Clipboard,
    clipboard_history: &mut Vec<ClipboardItem>,
) {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);

    if let Ok(new_copied_item) = clipboard.get_text() {
        clipboard_history.insert(0, ClipboardItem::new(new_copied_item));
    } else {
        println!("Error getting copied item");
    }

    if clipboard_history.len() > 4 {
        clipboard_history.pop();
    }
}

pub fn set_clipboard(
    clipboard: &mut Clipboard,
    clipboard_history: &mut Vec<ClipboardItem>,
    index: usize,
) {
    if index >= clipboard_history.len() {
        return;
    }

    let item = &clipboard_history[index];

    if let Err(error) = clipboard.set_text(item.content.to_owned()) {
        println!("Error setting clipboard item: {}", error);
    }
}

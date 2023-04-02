use std::{thread, time, process};

use rdev::{listen, Event, Key};
use rdev::EventType::KeyPress;
use arboard::Clipboard;

fn main() {
    listen_for_keypress();
}


fn store_clipboard_contents(clipboard: &mut Clipboard, clipboard_history: &mut Vec<ClipboardItem>) {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);

    let new_copied_item = clipboard.get_text();

    match new_copied_item {
        Ok(item) => clipboard_history.insert(0, ClipboardItem::new(item)),
        Err(err) => println!("Error getting copied item {err}"),
    }

    if clipboard_history.len() > 4 { clipboard_history.pop(); }
}

fn set_clipboard(clipboard: &mut Clipboard, clipboard_history: &mut Vec<ClipboardItem>, index: usize) {
    if index >= clipboard_history.len() { return };

    let item = &clipboard_history[index];
    
    if let Err(error) = clipboard.set_text(item.content.to_owned()) {
        println!("Error setting clipboard item {error}");
    }
}

fn listen_for_keypress() {
    let mut clipboard = Clipboard::new().unwrap_or_else(|err| { 
        eprintln!("Error creating clipboard context: {err}");
        process::exit(1)
    });
    
    let mut clipboard_history: Vec<ClipboardItem> = Vec::new();
    let mut previous_press_is_cmd: bool = false;
    let mut previous_press_is_ctrl: bool = false;

    if let Err(error) = listen(move |event| callback(event,&mut clipboard, &mut clipboard_history, &mut previous_press_is_cmd, &mut previous_press_is_ctrl)) {
        println!("Error {:?}", error);
    }

    fn callback(event: Event, clipboard: &mut Clipboard,  clipboard_history: &mut Vec<ClipboardItem>, previous_press_is_cmd: &mut bool, previous_press_is_ctrl: &mut bool) {

        match event.event_type {
            KeyPress(key) => {
                match key {         
                    Key::MetaLeft => { *previous_press_is_cmd = true },
                    Key::KeyC if *previous_press_is_cmd => { store_clipboard_contents(clipboard, clipboard_history) },
                    Key::ControlLeft => { *previous_press_is_ctrl = true },
                    Key::Num1 if *previous_press_is_ctrl => { set_clipboard(clipboard, clipboard_history, 0) },
                    Key::Num2 if *previous_press_is_ctrl => { set_clipboard(clipboard, clipboard_history, 1) },
                    Key::Num3 if *previous_press_is_ctrl => { set_clipboard(clipboard, clipboard_history, 2) },
                    Key::Num4 if *previous_press_is_ctrl => { set_clipboard(clipboard, clipboard_history, 3) },
                    _ => { *previous_press_is_cmd = false; *previous_press_is_ctrl = false  }
                }
            }
            _ => ()
        }
    }
    
}

#[derive(Debug)]
struct ClipboardItem {
    content: String
}

impl ClipboardItem {
    fn new(content: String) -> ClipboardItem {
        ClipboardItem { content }
    }
}
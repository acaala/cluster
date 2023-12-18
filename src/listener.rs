use std::process;

use crate::clipboard::{set_clipboard, store_clipboard_contents, ClipboardItem};
use arboard::Clipboard;
use rdev::{listen, Event, EventType, Key};

pub fn listen_for_keypress() {
    let mut clipboard = Clipboard::new().unwrap_or_else(|err| {
        eprintln!("Error creating clipboard context: {}", err);
        process::exit(1);
    });

    let mut clipboard_history: Vec<ClipboardItem> = Vec::new();
    let mut previous_press_is_cmd = false;
    let mut previous_press_is_ctrl = false;

    if let Err(error) = listen(move |event| {
        callback(
            event,
            &mut clipboard,
            &mut clipboard_history,
            &mut previous_press_is_cmd,
            &mut previous_press_is_ctrl,
        )
    }) {
        println!("Error: {:?}", error);
    }
}

fn callback(
    event: Event,
    clipboard: &mut Clipboard,
    clipboard_history: &mut Vec<ClipboardItem>,
    previous_press_is_cmd: &mut bool,
    previous_press_is_ctrl: &mut bool,
) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            Key::MetaLeft => {
                *previous_press_is_cmd = true;
            }
            Key::KeyC if *previous_press_is_cmd => {
                store_clipboard_contents(clipboard, clipboard_history);
            }
            Key::ControlLeft => {
                *previous_press_is_ctrl = true;
            }
            Key::Num1 if *previous_press_is_ctrl => {
                set_clipboard(clipboard, clipboard_history, 0);
            }
            Key::Num2 if *previous_press_is_ctrl => {
                set_clipboard(clipboard, clipboard_history, 1);
            }
            Key::Num3 if *previous_press_is_ctrl => {
                set_clipboard(clipboard, clipboard_history, 2);
            }
            Key::Num4 if *previous_press_is_ctrl => {
                set_clipboard(clipboard, clipboard_history, 3);
            }
            _ => {
                *previous_press_is_cmd = false;
                *previous_press_is_ctrl = false;
            }
        },
        _ => (),
    }
}

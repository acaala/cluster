mod clipboard;
mod listener;

use listener::listen_for_keypress;

fn main() {
    listen_for_keypress();
}

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

use std::io::Result;

use crate::Message;

fn try_read_key() -> Result<Option<KeyCode>> {
    // Set terminal to raw mode
    let mut res = Ok(None);
    // Read next event
    if let Event::Key(KeyEvent {
        code,
        modifiers,
        kind: _,
        state: _,
    }) = read()?
    {
        match code {
            KeyCode::Char(c) => {
                if modifiers == KeyModifiers::CONTROL && c == 'c' {
                    panic!("Ctrl + C");
                }
                if modifiers == KeyModifiers::CONTROL && (c == 'w' || c == 'h') {
                    res = Ok(Some(KeyCode::Backspace));
                } else {
                    res = Ok(Some(code));
                }
            }
            _ => {
                res = Ok(Some(code));
            }
        }
    }
    res
}

pub fn get_input_message() -> Option<Message> {
    let res = try_read_key();
    match res {
        Ok(Some(code)) => Some(Message::PressedKey(code)),
        Ok(None) => None,
        Err(e) => {
            if e.to_string() == "Ctrl + c" {
                Some(Message::Quit)
            } else {
                panic!("Error during input processing: {} ", e)
            }
        }
    }
}

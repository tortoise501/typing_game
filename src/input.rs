use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use std::{io::Result, process::exit};

use crate::Message;

fn try_read_key() -> Result<Option<InputSignal>> {
    // Set terminal to raw mode
    // let mut res = Ok(None);
    // Read next event
    if let Event::Key(key) = read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers == KeyModifiers::CONTROL && c == 'c' {
                    return Ok(Some(InputSignal::TerminateProgram));
                }
                if key.modifiers == KeyModifiers::CONTROL && (c == 'w' || c == 'h') {
                    return Ok(Some(InputSignal::Key(KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: key.modifiers,
                        kind: key.kind,
                        state: key.state,
                    })));
                } else {
                    return Ok(Some(InputSignal::Key(key)));
                }
            }
            _ => {
                return Ok(Some(InputSignal::Key(key)));
            }
        }
    }
    Ok(None)
}
pub fn get_input_process_input() -> Option<InputSignal> {
    match try_read_key() {
        Ok(key) => key,
        Err(emsg) => {
            eprintln!("problem occurred during input processing\n{}", emsg);
            exit(0);
        }
    }
}
/* pub fn get_input_message() -> Option<Message> {
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
} */
#[derive(Debug)]
pub enum InputSignal {
    Key(KeyEvent),
    TerminateProgram,
}

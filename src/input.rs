
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

use std::io::Result;


fn try_read_key() -> Result<Option<KeyCode>> {
    // Set terminal to raw mode
    let mut res = Ok(None);
        // Read next event
    if let Event::Key(KeyEvent { code, modifiers, kind: _, state: _ }) = read()? {
        match code {
            KeyCode::Char(c) => {
                if modifiers == KeyModifiers::CONTROL && c == 'c'{
                    crossterm::terminal::disable_raw_mode()?;//brakes terminal without this line
                    panic!("Ctrl + C");
                }
                res =  Ok(Some(code));
            },
            _ => {
                res = Ok(Some(code));
            }
        }
    }
    res
}

pub fn read_key() -> Option<KeyCode>{
    let res = try_read_key();
    match res {
        Ok(Some(code)) => {
            return Some(code);
        },
        Ok(None) =>{
            return None;
        },
        Err(e) => {
            panic!("Problem processing input, {}", e)
        }
    }
}
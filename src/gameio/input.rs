
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};


fn try_read_key() -> std::io::Result<Option<char>> {
    // Set terminal to raw mode
    crossterm::terminal::enable_raw_mode()?;
    let res;
    loop {
        // Read next event
        if let Event::Key(KeyEvent { code, modifiers, kind: _, state: _ }) = read()? {
            match code {
                KeyCode::Char(c) => {
                    if modifiers == KeyModifiers::CONTROL && c == 'c'{
                        crossterm::terminal::disable_raw_mode()?;//brakes terminal without this line
                        panic!("Ctrl + C pressed");//TODO: make better termination
                    }
                    res =  Ok(Some(c));
                    break;
                },
                KeyCode::Backspace => {
                    res = Ok(Some('\u{232B}'));
                    break;
                },
                _ => {
                    res = Ok(None);
                    break;
                }
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    res
}

pub fn read_key() -> Option<char>{
    let res = try_read_key();
    match res {
        Ok(Some(c)) => {
            return Some(c);
        },
        Ok(None) =>{
            return None;
        },
        Err(e) => {
            panic!("Problem processing input, {}", e)
        }
    }
}
use crate::{TextStatus,TypedString};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;


pub fn print_game_text(text_vec:&Vec<TypedString>) -> std::io::Result<&'static str>{
    // using the macro
    let print_as = |text:&TypedString| -> std::io::Result<&'static str> {

        let foreground_color = match text.text_status {
            TextStatus::Unfilled => Color::Grey,
            TextStatus::Filled => Color::White,
            TextStatus::Wrong => Color::Red,
        };
        let background_color = Color::Black;

        execute!(
            stdout(),
            Clear(ClearType::All),
            SetForegroundColor(foreground_color),
            SetBackgroundColor(background_color),
            Print(text.text.as_str()),
            ResetColor
        )?;
        Ok("ok")
    };
    for one_status_string in text_vec{
        print_as(&one_status_string)?;
    }
    
    
    Ok("ok")
}
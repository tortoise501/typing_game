// use crossterm::cursor::MoveTo;
// use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
// use crossterm::execute;
// use crossterm::terminal::{Clear, ClearType};
// use std::io::stdout;

// use crate::game::{LetterState,Letter};


// pub fn print_game_text(text_vec:&Vec<Letter>) -> std::io::Result<&'static str>{
//     // using the macro
//     execute!{
//         stdout(),
//         Clear(ClearType::All),
//         MoveTo(10, 10),//testing
//     }?;
//     for letter in text_vec{
//         let foreground_color = match letter.state {
//             LetterState::Unfilled => Color::Grey,
//             LetterState::Correct => Color::Green,
//             LetterState::Wrong => Color::Red,
//         };
//         let background_color = Color::Black;

//         execute!(
//             stdout(),
//             SetForegroundColor(foreground_color),
//             SetBackgroundColor(background_color),
//             Print(letter.c),
//             ResetColor
//         )?;
//     }
    
//     Ok("ok")
// }

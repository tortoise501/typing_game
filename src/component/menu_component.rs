use std::time::Duration;

use crate::game;
use crate::{component::game_conf_component, game::GameConf};

use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use ratatui::{self, widgets::Padding};

#[derive(Debug)]
pub struct MenuComp {
    current_opt: MenuOptions,
}
#[derive(Debug, PartialEq, FromPrimitive, Clone, Copy)]
pub enum MenuOptions {
    GameConf = 0,
    ExitProgram = 1,
}
impl MenuOptions {
    pub fn go_next(&mut self) {
        let i = *self as i32 + 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => MenuOptions::GameConf,
        }
    }
    pub fn go_prev(&mut self) {
        let i = *self as i32 - 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => MenuOptions::ExitProgram,
        }
    }
}
impl MenuComp {
    pub fn new() -> MenuComp {
        MenuComp {
            current_opt: MenuOptions::GameConf,
        }
    }
}
impl Component for MenuComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::KeyInput(key) => match key.code {
                // KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char(' ') | KeyCode::Enter => match self.current_opt {
                    MenuOptions::GameConf => Some(Message::GoToWindow(
                        WindowType::GameConfigMenu(GameConfigComp {
                            game_conf: game::GameConf::new(),
                            option: game_conf_component::SelectedOption::Mode,
                        }), //TODO: create new function with default game conf
                    )),
                    MenuOptions::ExitProgram => Some(Message::Quit),
                },
                KeyCode::Down => {
                    self.current_opt.go_next();
                    None
                }
                KeyCode::Up => {
                    self.current_opt.go_prev();
                    None
                }
                _ => None,
            },
            _ => None,
        };
        match answer {
            Some(a) => a,
            None => msg,
        }
    }

    fn view(&mut self, f: &mut Frame) {
        let lines = vec![
            Line::from(Span::styled(
                "Start rewrite",
                if self.current_opt == MenuOptions::GameConf {
                    Style::new().black().on_white()
                } else {
                    Style::new()
                },
            )),
            Line::from(Span::styled(
                "Exit program",
                if self.current_opt == MenuOptions::ExitProgram {
                    Style::new().black().on_white()
                } else {
                    Style::new()
                },
            )),
        ];
        f.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::new()
                        .title("Menu")
                        .borders(Borders::ALL)
                        .padding(Padding::vertical(3)),
                )
                .alignment(Alignment::Center)
                .style(Style::new().white().on_black())
                .bold()
                .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}

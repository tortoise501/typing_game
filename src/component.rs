use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::game::{Game, LetterState};

use crate::Message;

pub trait Component {
    fn handle_message(&mut self, msg: Message) -> Message;

    fn view(&mut self, f: &mut Frame);
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum ViewType {
    Menu(MenuComp),
    Game(GameComp),
    Statistics(StatComp),
}
impl Component for ViewType {
    fn handle_message(&mut self, msg: Message) -> Message {
        match self {
            ViewType::Menu(comp) => {
                comp.handle_message(msg)
            }
            ViewType::Game(comp) => {
                comp.handle_message(msg)
            }
            ViewType::Statistics(comp) => {
                comp.handle_message(msg)
            }
        }
    }

    fn view(&mut self, f: &mut Frame) {
        match self {
            ViewType::Menu(comp) => {
                comp.view(f);
            }
            ViewType::Game(comp) => {
                comp.view(f);
            }
            ViewType::Statistics(comp) => {
                comp.view(f);
            }
        };
    }
}
#[derive(Debug)]
pub struct MenuComp;
impl Component for MenuComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::PressedKey(code) => {
                match code {
                    KeyCode::Esc => {
                        Some(Message::Quit)
                    },
                    KeyCode::Char(' ') => {
                        Some(Message::StartGame)
                    },
                    _ => {
                        None
                    },
                }
            },
            _ => None,
        };
        match answer {
            Some(a) => a,
            None => msg,
        }
    }

    fn view(&mut self, f: &mut Frame) {
        f.render_widget(
            Paragraph::new("Press 'Space' to start the game, press 'Esc' to exit the game")
                .block(Block::new().title("Paragraph").borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}
#[derive(Debug)]
pub struct GameComp {
    pub(crate) game: Game,
}
impl Component for GameComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer: Option<Message> = match msg {
            Message::PressedKey(code) => {
                match code {
                    KeyCode::Esc => {
                        Some(Message::StopGame)
                    }
                    KeyCode::Char(c) => {
                        self.game.char_key_pressed(c);
                        if self.game.is_complete() {return Message::StopGame};
                        None
                    },
                    KeyCode::Backspace => {
                        self.game.backspace_pressed();
                        None
                    }
                    _ => {
                        None
                    }
                }
            },
            _ => None,
        };
        match answer {
            Some(a) => a,
            None => msg,
        }

    }

    fn view(&mut self, f: &mut Frame) {
        let matched_letter_vec = self.game.get_written_vec();

        let mut text: Vec<Span> = Vec::new();
        let mut unfilled_started = false;
        for letter in matched_letter_vec {
            text.push(
                Span::styled(
                    format!("{}", letter.c),
                    match letter.state {
                        LetterState::Unfilled if !unfilled_started => { unfilled_started = true; Style::new().on_gray().black() },
                        LetterState::Unfilled  => { Style::new().gray() },
                        LetterState::Correct => { Style::new().green() }
                        LetterState::Wrong => { Style::new().red() }
                    },
                )
            );
        }
        let text: Line = Line::from(text);

        f.render_widget(
            Paragraph::new(text)
                .block(Block::new().title("Paragraph").borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}
#[derive(Debug)]
pub struct StatComp {
    pub game: Game,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for StatComp{
    fn handle_message(&mut self, msg: Message) -> Message {
        todo!()
    }

    fn view(&mut self, f: &mut Frame) {
        todo!()
    }
}
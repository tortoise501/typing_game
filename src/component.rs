use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::game::{Game, LetterState};

use crate::Message;

pub trait Component {
    fn handle_message(&mut self, msg: Message) -> Option<Message>;

    fn view(&mut self, f: &mut Frame);
}
#[allow(dead_code)]
pub enum ViewType {
    Menu(MenuComp),
    Game(GameComp),
    Statistics(StatComp),
}
impl Component for ViewType {
    fn handle_message(&mut self, msg: Message) -> Option<Message> {
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

pub struct MenuComp;
impl Component for MenuComp {
    fn handle_message(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::PressedKey(code) => {
                return match code {
                    KeyCode::Esc => {
                        Some(Message::Quit)
                    }
                    KeyCode::Char(' ') => {
                        Some(Message::StartGame)
                    }
                    _ => {
                        None
                    }
                };
            }
            _ => None,
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

pub struct GameComp {
    pub(crate) game: Game,
}
impl Component for GameComp {
    fn handle_message(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::PressedKey(code) => {
                return match code {
                    KeyCode::Esc => {
                        Some(Message::StopGame)
                    }
                    KeyCode::Char(c) => {
                        self.game.char_key_pressed(c);
                        None
                    },
                    KeyCode::Backspace => {
                        self.game.backspace_pressed();
                        None
                    }
                    _ => {
                        None
                    }
                };
            }
            _ => None,
        }
    }

    fn view(&mut self, f: &mut Frame) {
        let matched_letter_vec = self.game.get_written_vec();

        let mut text: Vec<Span> = Vec::new();
        for letter in matched_letter_vec {
            text.push(
                Span::styled(
                    format!("{}", letter.c),
                    match letter.state {
                        LetterState::Unfilled => { Style::new().gray() }
                        LetterState::Correct => { Style::new().white() }
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

pub struct StatComp;
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for StatComp{
    fn handle_message(&mut self, msg: Message) -> Option<Message>{
        todo!()
    }

    fn view(&mut self, f: &mut Frame) {
        todo!()
    }
}
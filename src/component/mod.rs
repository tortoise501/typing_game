use crate::game::{Game, LetterState};
use crossterm::event::KeyCode;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::Message;

pub trait Component {
    fn handle_message(&mut self, msg: Message) -> Message;

    fn view(&mut self, f: &mut Frame);
}
pub mod menu_component;
pub use menu_component::MenuComp;

pub mod game_component;
pub use game_component::GameComp;

pub mod statistic_component;
pub use statistic_component::StatComp;

#[allow(dead_code)]
#[derive(Debug)]
pub enum WindowType {
    Menu(MenuComp),
    Game(GameComp),
    Statistics(StatComp),
}
impl Component for WindowType {
    fn handle_message(&mut self, msg: Message) -> Message {
        match self {
            WindowType::Menu(comp) => comp.handle_message(msg),
            WindowType::Game(comp) => comp.handle_message(msg),
            WindowType::Statistics(comp) => comp.handle_message(msg),
        }
    }

    fn view(&mut self, f: &mut Frame) {
        match self {
            WindowType::Menu(comp) => {
                comp.view(f);
            }
            WindowType::Game(comp) => {
                comp.view(f);
            }
            WindowType::Statistics(comp) => {
                comp.view(f);
            }
        };
    }
}

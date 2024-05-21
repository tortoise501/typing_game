use crate::game::{FieldState, Game};
use crossterm::event::KeyCode;
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::Message;
/// component represents window, its behavior and rendering
pub trait Component {
    /// processes message, reacts to it and answers with another message
    fn handle_message(&mut self, msg: Message) -> Message;

    /// renders component in set frame
    fn view(&mut self, f: &mut Frame);
}
pub mod menu_component;
pub use menu_component::MenuComp;

pub mod game_component;
pub use game_component::GameComp;

pub mod statistic_component;
pub use statistic_component::StatComp;

pub mod game_conf_component;
pub use game_conf_component::GameConfigComp;

/// enum representing witch window is active
#[allow(dead_code)]
#[derive(Debug)]
pub enum WindowType {
    Menu(MenuComp),
    Game(GameComp),
    Statistics(StatComp),
    GameConfigMenu(GameConfigComp),
}
impl WindowType {
    // gets itself as a component - crunch
    fn get_as_comp(&mut self) -> &mut dyn Component {
        match self {
            WindowType::Menu(comp) => comp,
            WindowType::Game(comp) => comp,
            WindowType::Statistics(comp) => comp,
            WindowType::GameConfigMenu(comp) => comp,
        }
    }
}
impl Component for WindowType {
    fn handle_message(&mut self, msg: Message) -> Message {
        self.get_as_comp().handle_message(msg)
    }

    fn view(&mut self, f: &mut Frame) {
        self.get_as_comp().view(f)
    }
}

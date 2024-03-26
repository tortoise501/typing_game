use ratatui::layout::{Constraint, Layout};

use super::*;
use crate::game::{GameConf, GameMode, Limit};

#[derive(Debug)]
pub struct GameConfigComp {
    pub game_conf: Option<GameConf>,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for GameConfigComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::KeyInput(key) => match key.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Enter => Some(Message::GoToWindow(WindowType::Menu(MenuComp::new()))),
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
        //Rendering border
        f.render_widget(Block::new().title("Border").borders(Borders::ALL), f.size());

        // +--------------------------------+
        // |  rewrite  normal               |
        // |  time  words  text             |
        // |  limit:{time|count|text_path}  |
        // +--------------------------------+
        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(Constraint::from_lengths([1, f.size().height - 2, 1]))
            .split(f.size());
        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_lengths([1, f.size().width - 2, 1]))
            .split(content_layout[1]);

        let selectors_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)]))
            .split(content_layout[1]);

        let mut render = |text: &str, rect: &ratatui::layout::Rect| {
            f.render_widget(Block::new().borders(Borders::ALL), *rect);
            let box_top_padding = (rect.height as f32 / 2 as f32).round() as u16 - 1;
            let rect = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(Constraint::from_lengths([box_top_padding, 1]))
                .split(*rect)[1];
            f.render_widget(Paragraph::new(text).alignment(Alignment::Center), rect);
        };

        let mode_selector_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_ratios([(1, 2), (1, 2)]))
            .split(selectors_layout[0]);

        render("normal", &mode_selector_layout[0]);
        render("rewrite", &mode_selector_layout[1]);

        let limit_selector_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)]))
            .split(selectors_layout[1]);

        render("time", &limit_selector_layout[0]);
        render("word count", &limit_selector_layout[1]);
        render("custom text", &limit_selector_layout[2]);

        let limit_input_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_percentages([100]))
            .split(selectors_layout[2]);

        render("input", &limit_input_layout[0]);
    }
}

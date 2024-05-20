use std::cell::RefCell;
use std::rc;
use std::{fmt::format, option, time::Duration};

use ratatui::layout::{Constraint, Layout};

use num_derive::FromPrimitive;
use num_traits::{clamp_max, FromPrimitive};
use ratatui::widgets::canvas::Line;

use super::*;
use crate::game::{GameConf, GameMode, Limit};

#[derive(Debug)]
pub struct GameConfigComp {
    pub game_conf: GameConf,
    pub option: SelectedOption,
}
#[derive(Debug, PartialEq, FromPrimitive, Clone, Copy)]
pub enum SelectedOption {
    Mode,
    Limit,
    Input,
}
type RCell<T> = rc::Rc<RefCell<T>>;
impl SelectedOption {
    pub fn next(&mut self) {
        let i = *self as i32 + 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => SelectedOption::Mode,
        }
    }
    pub fn prev(&mut self) {
        let i = *self as i32 - 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => SelectedOption::Input,
        }
    }
    fn right(&mut self, conf: &mut GameConf) {
        match self {
            SelectedOption::Mode => {
                conf.mode = match conf.mode {
                    GameMode::Normal => GameMode::Rewrite,
                    GameMode::Rewrite => GameMode::Normal,
                };
            }
            SelectedOption::Limit => {
                conf.limit = match conf.limit {
                    Limit::Time(_) => Limit::WordCount(50),
                    Limit::WordCount(_) => Limit::None,
                    Limit::None => Limit::Time(Duration::from_secs(30)),
                };
            }
            SelectedOption::Input => (),
        }
    }
    fn left(&mut self, conf: &mut GameConf) {
        match self {
            SelectedOption::Mode => {
                conf.mode = match conf.mode {
                    GameMode::Normal => GameMode::Rewrite,
                    GameMode::Rewrite => GameMode::Normal,
                };
            }
            SelectedOption::Limit => {
                conf.limit = match conf.limit {
                    Limit::Time(_) => Limit::None,
                    Limit::WordCount(_) => Limit::Time(Duration::from_secs(30)),
                    Limit::None => Limit::WordCount(50),
                };
            }
            SelectedOption::Input => (),
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for GameConfigComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        // let mut conf = &mut self.game_conf;
        let answer = match msg {
            Message::KeyInput(key) => match key.code {
                KeyCode::Esc => Some(Message::GoToWindow(WindowType::Menu(MenuComp::new()))),
                KeyCode::Down => {
                    self.option.next();
                    None
                }
                KeyCode::Up => {
                    self.option.prev();
                    None
                }
                KeyCode::Left => {
                    self.option.left(&mut self.game_conf);
                    None
                }
                KeyCode::Right => {
                    self.option.right(&mut self.game_conf);
                    None
                }
                KeyCode::Backspace => {
                    match &mut self.game_conf.limit {
                        Limit::Time(t) => {
                            *t = Duration::from_secs(t.as_secs() / 10);
                            None
                        }
                        Limit::WordCount(wc) => {
                            *wc = *wc / 10;
                            None
                        }
                        Limit::None => None, //TODO: path for custom file
                    }
                }
                KeyCode::Char(c) if self.option == SelectedOption::Input => {
                    match &mut self.game_conf.limit {
                        Limit::Time(t) => {
                            if c.is_numeric() {
                                let mut sec = t.as_secs();
                                if sec < 600 {
                                    //600 seconds is time limit, crunch to avoid too big numbers
                                    sec *= 10;
                                    sec += c.to_digit(10).unwrap() as u64;
                                    *t = Duration::from_secs(clamp_max(sec, 600))
                                } else {
                                    *t = Duration::from_secs(600)
                                }
                            }
                            None
                        }
                        Limit::WordCount(wc) => {
                            if c.is_numeric() {
                                if *wc < 10000 {
                                    //10000 is max word count, crunch to avoid too big numbers
                                    let mut count = *wc * 10;
                                    count += c.to_digit(10).unwrap() as u32;
                                    *wc = clamp_max(count, 10000);
                                }
                            }
                            None
                        }
                        Limit::None => None, //TODO: path for custom file
                    }
                }
                KeyCode::Enter if self.option == SelectedOption::Input => {
                    Some(Message::StartGame(self.game_conf.clone()))
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
            .split(f.size()); //top and bottom margins
        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_lengths([1, f.size().width - 2, 1]))
            .split(content_layout[1]); //left and right margins

        let selectors_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)]))
            .split(content_layout[1]);

        let mut render =
            |text: &str, rect: &ratatui::layout::Rect, color: ratatui::style::Color| {
                f.render_widget(Block::new().borders(Borders::ALL), *rect);
                let box_top_padding = (rect.height as f32 / 2 as f32).round() as u16 - 1;
                let rect = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints(Constraint::from_lengths([box_top_padding, 1]))
                    .split(*rect)[1];
                let style = if color == ratatui::style::Color::Black {
                    Style::new().on_black().white()
                } else {
                    Style::new().black().bg(color)
                };
                let text = Span::raw(text).style(style);
                f.render_widget(Paragraph::new(text).alignment(Alignment::Center), rect);
            };

        let mode_selector_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_ratios([(1, 2), (1, 2)]))
            .split(selectors_layout[0]);

        let limit_selector_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)]))
            .split(selectors_layout[1]);

        let limit_input_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(Constraint::from_percentages([100]))
            .split(selectors_layout[2]);
        let input_text = format!("limit:{:?}", &self.game_conf.limit);

        {
            render(
                "normal",
                &mode_selector_layout[0],
                ratatui::style::Color::Black,
            );
            render(
                "rewrite",
                &mode_selector_layout[1],
                ratatui::style::Color::Black,
            );
            render(
                "time",
                &limit_selector_layout[0],
                ratatui::style::Color::Black,
            );
            render(
                "word count",
                &limit_selector_layout[1],
                ratatui::style::Color::Black,
            );
            render(
                "custom text",
                &limit_selector_layout[2],
                ratatui::style::Color::Black,
            );
            render(
                input_text.as_str(),
                &limit_input_layout[0],
                ratatui::style::Color::Black,
            );
        }

        //rendering set settings for limits
        match self.game_conf.limit {
            Limit::Time(_) => render(
                "time",
                &limit_selector_layout[0],
                ratatui::style::Color::Green,
            ),
            Limit::WordCount(_) => render(
                "word count",
                &limit_selector_layout[1],
                ratatui::style::Color::Green,
            ),
            Limit::None => render(
                "custom text",
                &limit_selector_layout[2],
                ratatui::style::Color::Green,
            ),
        }
        //rendering set settings for mode   ratatui::style::Color::Black
        match self.game_conf.mode {
            GameMode::Normal => render(
                "normal",
                &mode_selector_layout[0],
                ratatui::style::Color::Green,
            ),
            GameMode::Rewrite => render(
                "rewrite",
                &mode_selector_layout[1],
                ratatui::style::Color::Green,
            ),
        }

        match &self.option {
            SelectedOption::Mode => match self.game_conf.mode {
                GameMode::Normal => render(
                    "normal",
                    &mode_selector_layout[0],
                    ratatui::style::Color::White,
                ),
                GameMode::Rewrite => render(
                    "rewrite",
                    &mode_selector_layout[1],
                    ratatui::style::Color::White,
                ),
            },
            SelectedOption::Limit => match self.game_conf.limit {
                Limit::Time(_) => render(
                    "time",
                    &limit_selector_layout[0],
                    ratatui::style::Color::White,
                ),
                Limit::WordCount(_) => render(
                    "word count",
                    &limit_selector_layout[1],
                    ratatui::style::Color::White,
                ),
                Limit::None => render(
                    "custom text",
                    &limit_selector_layout[2],
                    ratatui::style::Color::White,
                ),
            },
            SelectedOption::Input => render(
                input_text.as_str(),
                &limit_input_layout[0],
                ratatui::style::Color::White,
            ),
        }
    }
}

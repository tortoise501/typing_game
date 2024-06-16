use std::time::Duration;

use ratatui::layout::{Constraint, Layout};

use num_derive::FromPrimitive;
use num_traits::{clamp, FromPrimitive};

use super::*;
use crate::game::{GameConf, GameMode, Limit};
/// component responsible for configuration window
#[derive(Debug)]
pub struct GameConfigComp {
    pub game_conf: GameConf,
    pub option: SelectedOption,
}
///configuration option selected for input
#[derive(Debug, PartialEq, FromPrimitive, Clone, Copy)]
pub enum SelectedOption {
    Mode,
    Limit,
    Input,
}
impl SelectedOption {
    ///select next option
    pub fn next(&mut self) {
        let i = *self as i32 + 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => SelectedOption::Mode,
        }
    }
    ///select previous option
    pub fn prev(&mut self) {
        let i = *self as i32 - 1;
        *self = match FromPrimitive::from_i32(i) {
            Some(opt) => opt,
            None => SelectedOption::Input,
        }
    }
    ///select next type of this option
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

    ///select previous type of this option
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
    /// react to message and answer
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
                            *t = Duration::from_secs(clamp(t.as_secs() / 10, 0, 600));
                            None
                        }
                        Limit::WordCount(wc) => {
                            *wc = clamp(*wc / 10,0,10000);
                            None
                        }
                        Limit::None => None, //TODO: path for custom file
                    }
                }
                KeyCode::Char(c) if self.option == SelectedOption::Input => {
                    if c.is_numeric(){
                        match &mut self.game_conf.limit {
                            Limit::Time(t) => {
                                let mut sec = t.as_secs();
                                //600 seconds is time limit, crunch to avoid too big numbers
                                sec *= 10;
                                sec += c.to_digit(10).unwrap() as u64;
                                sec = clamp(sec, 1, 600);
                                *t = Duration::from_secs(clamp(sec, 1, 600))
                            },
                            Limit::WordCount(wc) => {
                                //10000 is max word count, crunch to avoid too big numbers
                                let mut count = *wc * 10;
                                count += c.to_digit(10).unwrap() as u32;
                                *wc = clamp(count, 1,10000);
                            },
                            Limit::None => todo!(),
                        }
                        None
                    }else{
                        None
                    }

                    // match &mut self.game_conf.limit {
                    //     Limit::Time(t) => {
                    //         if c.is_numeric() {
                    //             let mut sec = t.as_secs();
                    //             if sec < 600 {
                    //                 //600 seconds is time limit, crunch to avoid too big numbers
                    //                 sec *= 10;
                    //                 sec += c.to_digit(10).unwrap() as u64;
                    //                 *t = Duration::from_secs(clamp_max(sec, 600))
                    //             } else {
                    //                 *t = Duration::from_secs(600)
                    //             }
                    //         }
                    //         None
                    //     }
                    //     Limit::WordCount(wc) => {
                    //         if c.is_numeric() {
                    //             if *wc < 10000 {
                    //                 //10000 is max word count, crunch to avoid too big numbers
                    //                 let mut count = *wc * 10;
                    //                 count += c.to_digit(10).unwrap() as u32;
                    //                 *wc = clamp_max(count, 10000);
                    //             }
                    //         }
                    //         None
                    //     }
                    //     Limit::None => None, //TODO: path for custom file
                    // }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
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
    /// render game configuration window
    fn view(&mut self, f: &mut Frame) {
        //Rendering border
        f.render_widget(Block::new().title("Game configuration | Esc = go to menu | arrow buttons = navigation | Enter or Space = start the game").borders(Borders::ALL), f.size());

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

        let render =
            |text: &str, rect: &ratatui::layout::Rect, is_selected:bool,f: &mut Frame| {
                 if is_selected {
                    f.render_widget(Block::new().borders(Borders::ALL).style(Style::new().green()), *rect);
                } 
                let box_top_padding = (rect.height as f32 / 2 as f32).round() as u16 - 1;
                let rect = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints(Constraint::from_lengths([box_top_padding, 1]))
                    .split(*rect)[1];
                let text = Span::raw(text).style(Style::new().on_black().white());
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
                false,f
            );
            render(
                "rewrite",
                &mode_selector_layout[1],
                false,f            );
            render(
                "time",
                &limit_selector_layout[0],
                false,f            );
            render(
                "word count",
                &limit_selector_layout[1],
                false,f            );
            render(
                "custom text",
                &limit_selector_layout[2],
                false,f            );
            render(
                input_text.as_str(),
                &limit_input_layout[0],
                false,f            );
        }



        match &self.option {
            SelectedOption::Mode => {
                f.render_widget(Block::new().borders(Borders::ALL).style(Style::new().white()), selectors_layout[0]);

            },
            SelectedOption::Limit => {
                f.render_widget(Block::new().borders(Borders::ALL).style(Style::new().white()), selectors_layout[1]);

            },
            SelectedOption::Input => {
                f.render_widget(Block::new().borders(Borders::ALL).style(Style::new().white()), selectors_layout[2]);

            }
        }

        //rendering set settings for limits
        match self.game_conf.limit {
            Limit::Time(_) => render(
                "time",
                &limit_selector_layout[0],
                true,f 
            ),
            Limit::WordCount(_) => render(
                "word count",
                &limit_selector_layout[1],
                true,f            
            ),
            Limit::None => render(
                "custom text",
                &limit_selector_layout[2],
                true,f            
            ),
        }
        //rendering set settings for mode
        match self.game_conf.mode {
            GameMode::Normal => render(
                "normal",
                &mode_selector_layout[0],
                true,f            
            ),
            GameMode::Rewrite => render(
                "rewrite",
                &mode_selector_layout[1],
                true,f            
            ),
        }
    }
}

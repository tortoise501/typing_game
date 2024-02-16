use crossterm::event::KeyCode;
use std::io::Result;

use ratatui::Frame;

use game::Game;
use model::Model;

use crate::component::{Component, GameComp, MenuComp, StatComp, WindowType};
// use crate::Message::GameStopped;

mod component;
mod game;
mod input;
mod markov_gen;
mod model;
mod tui;

fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut game_model = Model {
        active_window: WindowType::Menu(MenuComp::new()),
        running_state: model::RunningState::Running,
    };
    while game_model.running_state == model::RunningState::Running {
        terminal.draw(|f| view(&mut game_model, f))?;

        let mut current_msg = input::get_input_message();
        while current_msg.is_some() {
            current_msg = update(&mut game_model, current_msg.unwrap());
        }
    }
    tui::restore_terminal()?;
    Ok(())
}
// #[allow(dead_code)]
#[derive(Debug)]
enum Message {
    PressedKey(KeyCode),
    StartGame,
    StopGame,
    GameStopped(Option<Game>),
    GoToWindow(WindowType),
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    let answer = model.active_window.handle_message(msg);
    process_answer(model, answer)
}

fn process_answer(model: &mut Model, answer: Message) -> Option<Message> {
    match answer {
        Message::StartGame => {
            model.active_window = WindowType::Game(GameComp {
                game: Game::new(100),
            });
            None
        }
        Message::StopGame => Some(match &mut model.active_window {
            WindowType::Game(comp) => Message::GameStopped(Some(comp.game.clone())),
            _ => Message::GameStopped(None),
        }),
        Message::Quit => {
            tui::restore_terminal().expect("TODO: panic message");
            panic!("TODO: better program termination");
        }
        Message::GameStopped(game) => match game {
            Some(game) => {
                if game.is_complete() {
                    Some(Message::GoToWindow(WindowType::Statistics(StatComp {
                        game,
                    })))
                } else {
                    Some(Message::GoToWindow(WindowType::Menu(MenuComp::new())))
                }
            }
            None => Some(Message::GoToWindow(WindowType::Menu(MenuComp::new()))),
        },
        Message::GoToWindow(window) => {
            model.active_window = window;
            None
        }
        _ => None,
    }
}

fn view(model: &mut Model, f: &mut Frame) {
    model.active_window.view(f);
}

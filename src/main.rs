use crossterm::event::KeyEvent;
use input::InputSignal;
use std::{io::Result, process::exit, sync::mpsc, thread, time::Duration};

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
enum OutsideMessage {
    Message(Message),
    InputSignal(Option<InputSignal>),
}
fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut game_model = Model {
        active_window: WindowType::Menu(MenuComp::new()),
        running_state: model::RunningState::Running,
    };
    let tick_delay = Duration::from_millis(100);
    let (tx_input, rx) = mpsc::channel();
    let tx_tick = tx_input.clone();
    let _tick_thread = Box::new(thread::spawn(move || loop {
        thread::sleep(tick_delay);
        _ = tx_tick.send(OutsideMessage::Message(Message::Tick));
    }));
    let _input_thread = thread::spawn(move || loop {
        _ = tx_input.send(OutsideMessage::InputSignal(input::get_input_process_input()));
    });

    while game_model.running_state == model::RunningState::Running {
        terminal.draw(|f| view(&mut game_model, f))?;

        // let input = input::get_input_process_input();
        let mut current_msg = match rx.recv() {
            //? Not Safe
            Ok(om) => match om {
                OutsideMessage::Message(msg) => Some(msg),
                OutsideMessage::InputSignal(sig) => match sig {
                    Some(s) => match s {
                        InputSignal::Key(key) => Some(Message::KeyInput(key)),
                        InputSignal::TerminateProgram => {
                            tui::restore_terminal()?;
                            exit(0)
                        }
                    },
                    None => None,
                },
            },
            Err(_) => {
                // println!("{:?}", e);
                // tick_thread.join();
                // input_thread.join();//IDl why this didn't work but it is probably not needed
                None
            }
        };

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
    KeyInput(KeyEvent),
    Tick,
    StartGame(game::GameConf),
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
        Message::StartGame(conf) => {
            model.active_window = WindowType::Game(GameComp {
                game: Game::new(20, conf, None), //TODO: not handling custom texts
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
                        result_text: None,
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

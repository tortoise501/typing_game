use crossterm::event::KeyEvent;
use input::InputSignal;
use std::{
    io::Result, sync::mpsc, thread, time::{Duration, SystemTime}
};

use ratatui::Frame;

use game::Game;
use model::Model;

use crate::component::{Component, GameComp, MenuComp, StatComp, WindowType};
// use crate::Message::GameStopped;

mod component;
mod game;
mod input;
mod model;
mod tui;
mod config_manager;
enum OutsideMessage {
    Message(Message),
    InputSignal(Option<InputSignal>),
}
fn main() -> Result<()> {
    tui::install_panic_hook();//something to fix terminal if closed with Ctrl+C
    let mut terminal = tui::init_terminal()?;
    // tui::restore_terminal_new(&mut terminal);
    // exit(0);
    let gen_text = config_manager::read_markov_text_file();
    let mut game_model = Model {
        active_window: WindowType::Menu(MenuComp::new()),
        running_state: model::RunningState::Running,
        gen_text
    };
    let tick_delay = Duration::from_millis(100);//delay between every game logic calculation and render
    let (tx_input, rx) = mpsc::channel();//create chanel to get input signals
    let tx_tick = tx_input.clone();//crete sender for tick signals
    let _tick_thread = Box::new(thread::spawn(move || loop {
        thread::sleep(tick_delay);
        _ = tx_tick.send(OutsideMessage::Message(Message::Tick));
    }));
    let _input_thread = thread::spawn(move || loop {
        _ = tx_input.send(OutsideMessage::InputSignal(input::get_input_process_input()));
    });

    'outer: while game_model.running_state == model::RunningState::Running {
        terminal.draw(|f| view(&mut game_model, f))?;
        // tui::restore_terminal_new(&mut terminal);
        // exit(0);
        // let input = input::get_input_process_input();
        let mut current_msg = match rx.recv() {
            //? Not Safe
            Ok(om) => match om {
                OutsideMessage::Message(msg) => Some(msg),
                OutsideMessage::InputSignal(sig) => match sig {
                    Some(s) => match s {
                        InputSignal::Key(key) => Some(Message::KeyInput(key)),
                        InputSignal::TerminateProgram => {
                            let _ = tui::restore_terminal(&mut terminal);
                            panic!("TODO: better program termination")
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
            if let Some(Message::Quit) = current_msg {
                break 'outer
            }
        }
    }
    let _ = tui::restore_terminal(&mut terminal);
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
            model.active_window = WindowType::Game(GameComp::new(
                Game::new(1000, conf, Some(model.gen_text.clone())),
                SystemTime::now(),
                Duration::from_secs(1), //TODO: make it configurable with config file
            ));
            None
        }
        Message::StopGame => Some(match &mut model.active_window {
            WindowType::Game(comp) => {
                comp.game.stop_game();
                Message::GameStopped(Some(comp.game.clone()))
            }
            _ => Message::GameStopped(None),
        }),
        Message::Quit => {
            Some(Message::Quit)
        }
        Message::GameStopped(game) => match game {
            Some(game) => {
                if game.is_complete() {
                    Some(Message::GoToWindow(WindowType::Statistics(StatComp {
                        game,
                        statistics: None,
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

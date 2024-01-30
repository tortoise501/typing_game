use std::io::Result;
use component::{MenuComp, StatComp};
use crossterm::event::KeyCode;

use ratatui::Frame;

use game::Game;
use model::Model;


use crate::component::{Component, GameComp, ViewType};
// use crate::Message::GameStopped;

mod game;
mod input;
mod model;
mod tui;
mod component;


fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut game_model = Model {
        active_view: ViewType::Menu(component::MenuComp),
        running_state: model::RunningState::Running,
    };
    while game_model.running_state == model::RunningState::Running {
        terminal.draw(|f| view(&mut game_model, f))?;

        let mut current_msg = handle_event()?;
        while current_msg.is_some() {
            current_msg = update(&mut game_model, current_msg.unwrap());
        }
    }
    tui::restore_terminal()?;
    Ok(())
}

//TODO: handle other events than keys
fn handle_event() -> Result<Option<Message>> {
    match input::read_key() {
        Some(code) => {
            Ok(Some(Message::PressedKey(code)))
        },
        _ => {
            Ok(None)
        }
    }
}
// #[allow(dead_code)]
#[derive(Debug)]
enum Message {
    PressedKey(KeyCode),
    StartGame,
    StopGame,
    GameStopped(Option<Game>),
    GoToView(ViewType),
    Quit,

}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    // msg = match process_message(model,msg){
    //     Some(m) => m,
    //     None => return None
    // };
    let answer = model.active_view.handle_message(msg);
    process_message(model,answer)
    
}

fn process_message(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::StartGame => {
            model.active_view = ViewType::Game( GameComp {
                game: Game::new(),
            });
            None
        }
        Message::StopGame => {
            Some(
                match &mut model.active_view {
                ViewType::Game(comp) => {
                    Message::GameStopped(Some(comp.game.clone()))
                },
                _ => Message::GameStopped(None),
            })
        },
        Message::Quit => {
            tui::restore_terminal().expect("TODO: panic message");
            panic!("TODO: better program termination");
        },
        Message::GameStopped(game) => {
            match game {
                Some(game) => {
                    if game.is_complete(){
                        Some(Message::GoToView(
                            ViewType::Statistics( StatComp {game} )
                        ))
                    } else {
                        Some(Message::GoToView(ViewType::Menu(MenuComp)))
                    }
                    
                },
                None => {
                    Some(Message::GoToView(ViewType::Menu(MenuComp)))
                }

            }
        },
        Message::GoToView(view) => {
            model.active_view = view;
            None
        },
        _ => None
    }
}



fn view(model: &mut Model, f: &mut Frame) {
    model.active_view.view(f);
}
use std::io::Result;
use crossterm::event::KeyCode;

use ratatui::{
    Frame,
    layout::Alignment,
    style::{Style, Stylize},
    text::{
        Line,
        Span,
    },
    widgets::{
        Block, Borders, Paragraph, Wrap,
    },
};

use game::Game;
use model::Model;

use crate::game::LetterState;
use crate::component::{Component, GameComp, MenuComp, ViewType};
use crate::Message::GameStopped;

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

enum Message {
    PressedKey(KeyCode),
    StartGame,
    StopGame,
    GameStopped(Option<Game>),
    QuitView,
    Quit,

}

fn update(model: &mut Model, msg: Message) -> Option<Message> {

    let answer = match &mut model.active_view {
        ViewType::Menu(comp) => {
            comp.handle_message(msg)
        }
        ViewType::Game(comp) => {
            comp.handle_message(msg)
        }
        ViewType::Statistics(comp) => {
            comp.handle_message(msg)
        }
    };
    let answer = match answer{
        Some(a) => a,
        None => return None,
    };
    match answer {
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
                    GameStopped(Some(comp.game.clone()))
                },
                _ => GameStopped(None),
            })
        },
        Message::Quit => {
            tui::restore_terminal().expect("TODO: panic message");
            panic!("TODO: better program termination");
        },
        _ => None
    }
}


fn view(model: &mut Model, f: &mut Frame) {
    match &mut model.active_view {
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
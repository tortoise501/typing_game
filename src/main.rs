use std::io::Result;

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
use crate::model::View;

mod game;
mod input;
mod model;
mod tui;
mod view;


fn main() -> Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut game_model = Model {
        game: Game::new(),
        active_view: model::View::Menu,
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

fn handle_event() -> Result<Option<Message>> {
    match input::read_key() {
        Some(c) if c == '\u{232B}' => {
            Ok(Some(Message::PressedBackspace))
        }
        Some(c) => {
            Ok(Some(Message::PressedCharKey(c)))
        }
        _ => {
            Ok(None)
        }
    }
}

enum Message {
    PressedCharKey(char),
    PressedBackspace,
    StopGame,
    ResetGame,
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {

    match model.active_view {
        View::Menu => {}
        View::Game => {
            match msg {
                Message::PressedCharKey(c) => {
                    model.game.char_key_pressed(c);
                }
                Message::PressedBackspace => {
                    model.game.backspace_pressed();
                }
                Message::StopGame => {
                    model.running_state = model::RunningState::Done;
                }
                Message::ResetGame => {
                    model.game = Game::new();
                }
                Message::Quit => {
                    model.running_state = model::RunningState::Done;
                }
            };
        }
        View::Statistics => {}
    }



    None
}


fn view(model: &mut Model, f: &mut Frame) {
    match model.active_view {
        View::Menu => {
            menu_view(model,f);
        }
        View::Game => {
            game_view(model,f);
        }
        View::Statistics => {}
    }


}

fn game_view(model: &mut Model, f: &mut Frame) {
    let matched_letter_vec = model.game.get_written_vec();

    let mut text: Vec<Span> = Vec::new();
    for letter in matched_letter_vec {
        text.push(
            Span::styled(
                format!("{}", letter.c),
                match letter.state {
                    LetterState::Unfilled => { Style::new().gray() }
                    LetterState::Correct => { Style::new().white() }
                    LetterState::Wrong => { Style::new().red() }
                },
            )
        );
    }
    let text: Line = Line::from(text);

    f.render_widget(
        Paragraph::new(text)
            .block(Block::new().title("Paragraph").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        f.size(),
    );
}

fn menu_view(model: &mut Model, f: &mut Frame){
    f.render_widget(
        Paragraph::new("Press 'Space' to start the game, press 'Esc' to exit the game")
            .block(Block::new().title("Paragraph").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        f.size(),
    );
}


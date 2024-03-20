use crossterm::event::KeyModifiers;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text,
};

use super::*;

#[derive(Debug)]
pub struct GameComp {
    pub(crate) game: Game,
}
impl Component for GameComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer: Option<Message> = match msg {
            Message::KeyInput(key) => match key.code {
                KeyCode::Esc => Some(Message::StopGame),
                KeyCode::Char(c) => {
                    self.game.char_key_pressed(c);
                    if self.game.is_complete() {
                        return Message::StopGame;
                    };
                    None
                }
                KeyCode::Backspace => {
                    match key.modifiers {
                        KeyModifiers::CONTROL => self.game.clear_last_world(),
                        _ => self.game.clear_last_letter(),
                    }
                    None
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
        match self.game.game_conf.mode {
            crate::game::GameMode::Normal => self.normal_view(f),
            crate::game::GameMode::Rewrite => self.rewrite_view(f),
        }
    }
}
impl GameComp {
    fn normal_view(&mut self, f: &mut Frame) {
        let write_field_rows = 3; //height of field where text is displayed //TODO: make it changeable in game settings

        let matched_letter_vec = self.game.get_written_vec();

        //how many letter to skip to allow wrapping //TODO: doesn't work well with wrapping, fix for wrapping (process words instead off letters?)
        let text_width = (f.size().width - 5) as usize;
        let filled_letters = matched_letter_vec
            .iter()
            .filter(|&x| x.state != FieldState::Unfilled)
            .count();
        let letters_to_skip = filled_letters
            - (filled_letters % (text_width * (write_field_rows as f32 / 2.0).round() as usize));

        let mut text: Vec<Span> = Vec::new();
        let mut unfilled_started = false;
        for letter in &matched_letter_vec[letters_to_skip..] {
            text.push(Span::styled(
                format!("{}", letter.c),
                match letter.state {
                    FieldState::Unfilled if !unfilled_started => {
                        unfilled_started = true;
                        Style::new().on_gray().black().not_underlined()
                    }
                    FieldState::Unfilled => Style::new().gray().not_underlined(),
                    FieldState::Correct => Style::new().green().not_underlined(),
                    FieldState::Wrong => {
                        if letter.c == ' ' {
                            Style::new()
                                .red()
                                .underlined()
                                .underline_color(ratatui::style::Color::Red)
                        } else {
                            Style::new().red().not_underlined()
                        }
                    }
                },
            ));
        }
        let text: Line = Line::from(text);

        let write_field_rows = write_field_rows as u16;
        //layout that divides screen on top, center and bottom rows
        let y_center_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((f.size().height - (write_field_rows + 2)) / 2),
                Constraint::Length(write_field_rows + 2),
                Constraint::Length((f.size().height - (write_field_rows + 2)) / 2),
            ])
            .split(f.size());
        //fully centered layout
        let centered_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(f.size().width - 3),
                Constraint::Length(1),
            ])
            .split(y_center_layout[1]);

        f.render_widget(Block::new().title("Border").borders(Borders::ALL), f.size());
        f.render_widget(
            Paragraph::new(text)
                .block(Block::new().borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            centered_layout[1],
        );
    }
    fn rewrite_view(&mut self, f: &mut Frame) {
        let matched_letter_vec = self.game.get_written_vec();

        let mut text: Vec<Span> = Vec::new();
        let mut unfilled_started = false;
        for letter in matched_letter_vec {
            text.push(Span::styled(
                format!("{}", letter.c),
                match letter.state {
                    FieldState::Unfilled if !unfilled_started => {
                        unfilled_started = true;
                        Style::new().on_gray().gray().not_underlined() //cursor
                    }
                    FieldState::Unfilled => break,
                    FieldState::Correct => Style::new().green().not_underlined(),
                    FieldState::Wrong => {
                        if letter.c == ' ' {
                            Style::new()
                                .red()
                                .underlined()
                                .underline_color(ratatui::style::Color::Red)
                        } else {
                            Style::new().red().not_underlined()
                        }
                    }
                },
            ));
        }
        let text: Line = Line::from(text);

        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(f.size());

        f.render_widget(
            Paragraph::new(text)
                .block(Block::new().title("You type here").borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            layout[0],
        );
        f.render_widget(
            Paragraph::new(
                self.game
                    .correct_text
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .concat(),
            )
            .block(
                Block::new()
                    .title("Rewrite this text")
                    .borders(Borders::ALL),
            )
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
            layout[1],
        );
    }
}

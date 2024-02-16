use ratatui::layout::{self, Constraint, Direction, Layout};

use super::*;

#[derive(Debug)]
pub struct GameComp {
    pub(crate) game: Game,
}
impl Component for GameComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer: Option<Message> = match msg {
            Message::PressedKey(code) => match code {
                KeyCode::Esc => Some(Message::StopGame),
                KeyCode::Char(c) => {
                    self.game.char_key_pressed(c);
                    if self.game.is_complete() {
                        return Message::StopGame;
                    };
                    None
                }
                KeyCode::Backspace => {
                    self.game.backspace_pressed();
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
        match self.game.game_mode {
            crate::game::GameMode::Normal => self.normal_view(f),
            crate::game::GameMode::Rewrite => self.rewrite_view(f),
        }
    }
}
impl GameComp {
    fn normal_view(&mut self, f: &mut Frame) {
        let matched_letter_vec = self.game.get_written_vec();

        let mut text: Vec<Span> = Vec::new();
        let mut unfilled_started = false;
        for letter in matched_letter_vec {
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

        f.render_widget(
            Paragraph::new(text)
                .block(Block::new().title("Paragraph").borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            f.size(),
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
                .block(Block::new().title("Paragraph").borders(Borders::ALL))
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
            .block(Block::new().title("Paragraph").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
            layout[1],
        );
    }
}

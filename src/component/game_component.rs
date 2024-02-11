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
}

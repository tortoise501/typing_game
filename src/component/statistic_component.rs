use super::*;

#[derive(Debug)]
pub struct StatComp {
    pub game: Game,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for StatComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::PressedKey(code) => match code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Enter => Some(Message::GoToWindow(WindowType::Menu(MenuComp))),
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
        f.render_widget(
            Paragraph::new(format!(
                "Your speed: {} wpm. At the end of the test you had {} mistakes.\nPress 'Enter' to go to menu or 'Esc' to exit the game.",
                2,
                self.game
                    .written_vec
                    .iter()
                    .filter(|letter| { letter.state == LetterState::Wrong })
                    .count()
            ))
            .block(Block::new().title("Paragraph").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}

use super::*;

#[derive(Debug)]
pub struct MenuComp;
impl Component for MenuComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::PressedKey(code) => match code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Char(' ') => Some(Message::StartGame),
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
            Paragraph::new("Press 'Space' to start the game, press 'Esc' to exit the game")
                .block(Block::new().title("Paragraph").borders(Borders::ALL))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}

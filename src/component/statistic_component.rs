use crate::game::GameStat;

use super::*;

#[derive(Debug)]
pub struct StatComp {
    pub game: Game,
    ///statistics
    pub statistics: Option<GameStat>,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Component for StatComp {
    fn handle_message(&mut self, msg: Message) -> Message {
        let answer = match msg {
            Message::KeyInput(key) => match key.code {
                KeyCode::Esc => Some(Message::Quit),
                KeyCode::Enter => Some(Message::GoToWindow(WindowType::Menu(MenuComp::new()))),
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
        // let stats = &self.game.get_statistics();
        let stats = if let Some(stats) = &self.statistics {
            stats.clone()
        } else {
            self.statistics = Some(self.game.get_statistics());
            self.game.get_statistics()
        };

        let result_text = format!(
            "Your total speed was: {} wpm.\nAt the end of the test you had {} words written wrong.\nYour accuracy is {:.2}.\nImagine this is a graph (WIP): {:?}\nPress 'Enter' to go to menu or 'Esc' to exit the game.\n\n\n\n\n debugging{:?}",
            stats.speed_stat.last().or(Some(&0)).unwrap(),
            stats.total_words - stats.correct_words,
            ((stats.correct_strokes as f32 / (stats.correct_strokes + stats.wrong_strokes) as f32) * 100.0),
            stats.speed_stat,
            stats
        );
        f.render_widget(
            Paragraph::new(result_text.clone())
                .block(
                    Block::new()
                        .title("Your game statistics")
                        .borders(Borders::ALL),
                )
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false }),
            f.size(),
        );
    }
}

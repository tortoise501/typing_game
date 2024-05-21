use num_traits::Zero;
use ratatui::layout::{Constraint, Layout};

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
        let zones_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.size());
        let number_area = zones_layout[0];
        let number_layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ],
        )
        .vertical_margin((number_area.height - 1) / 2)
        .split(number_area);

        let number_paragraph = |line: String| -> Paragraph {
            Paragraph::new(Line::from(line).alignment(Alignment::Center))
        };

        let time_spent_in_minutes = stats
            .time_finished
            .duration_since(stats.time_started)
            .unwrap()
            .as_secs_f32()
            / 60 as f32;

        f.render_widget(
            number_paragraph(format!(
                "Total words per minute: {}",
                (stats.correct_words as f32 / time_spent_in_minutes).round()
            )), //?SPEED
            number_layout[0],
        );
        f.render_widget(
            number_paragraph(format!(
                "Incorrectly typed words: {}",
                stats.total_words - stats.correct_words
            )),
            number_layout[1],
        );
        f.render_widget(
            number_paragraph(format!(
                "Accuracy: {}%",
                if stats.correct_strokes.is_zero() {
                    0
                } else {
                    (stats.wrong_strokes / stats.correct_strokes) * 100
                }
            )),
            number_layout[2],
        );

        let graph_zone = zones_layout[1];

        // let result_text = format!(
        //     "Your total speed was: {} wpm.\nAt the end of the test you had {} words written wrong.\nYour accuracy is {:.2}.\nImagine this is a graph (WIP): {:?}\nPress 'Enter' to go to menu or 'Esc' to exit the game.\n\n\n\n\n debugging{:?}",
        //     stats.speed_stat.last().or(Some(&0)).unwrap(),
        //     stats.total_words - stats.correct_words,
        //     ((stats.correct_strokes as f32 / (stats.correct_strokes + stats.wrong_strokes) as f32) * 100.0),
        //     stats.speed_stat,
        //     stats
        // );
        // f.render_widget(
        //     Paragraph::new(result_text.clone())
        //         .block(
        //             Block::new()
        //                 .title("Your game statistics")
        //                 .borders(Borders::ALL),
        //         )
        //         .style(Style::new().white().on_black())
        //         .alignment(Alignment::Left)
        //         .wrap(Wrap { trim: false }),
        //     f.size(),
        // );
    }
}

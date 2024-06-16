
use num_traits::Zero;
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::canvas::{Canvas, Map, MapResolution, Rectangle},
};

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
        let mut stats = if let Some(stats) = &self.statistics {
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
                    100 - f32::round((stats.wrong_strokes as f32 / (stats.correct_strokes + stats.wrong_strokes) as f32) * 100_f32) as u32
                }
            )),
            number_layout[2],
        );

        let graph_zone = zones_layout[1];
        let width = 110_f64 + 10.0;
        let height = 110_f64;
        let canvas = Canvas::default()
            .block(Block::bordered().title("Graph"))
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(|ctx| {
                // ctx.draw(&Map {
                //     resolution: MapResolution::High,
                //     color: Color::White,
                // });
                ctx.layer();
                let mut speed_stat_scaled: Vec<i32> = Vec::new();

                let block_width = width / stats.speed_stat.len() as f64;
                //makes vector shorter if too many elements
                if stats.speed_stat.len() > width as usize {
                    let scale = stats.speed_stat.len() as f64 / (width / block_width); //scale for stats to fit 20 blocks
                    let mut count = 0;
                    let mut sum = 0;
                    for interval in &stats.speed_stat {
                        count += 1;
                        sum += interval;
                        if ((scale - count as f64) / 10.0).round() * 10.0 == 0.0
                            || count == stats.speed_stat.len()
                        {
                            speed_stat_scaled.push(sum);
                            sum = 0;
                            count = 0;
                        }
                    }
                } else {
                    speed_stat_scaled = stats.speed_stat.iter().map(|x| -> i32 { *x }).collect();
                }

                let len = speed_stat_scaled.len();
                let mut speed_stat_scaled_stabilized: Vec<f64> = Vec::new();
                let mut sum = 0.0;
                //makes it more stable showing speed of all time up to the point on graph
                for (index, interval) in speed_stat_scaled.iter().enumerate() {
                    sum += *interval as f64;
                    speed_stat_scaled_stabilized.push(sum / (index as f64 + 1.0))
                }
                let speed_stat_scaled = speed_stat_scaled_stabilized;
                let mut max_interval = *speed_stat_scaled
                    .iter()
                    .max_by(|a, b| a.total_cmp(b))
                    .or(Some(&1.0))
                    .unwrap();
                if max_interval == 0.0 {
                    max_interval = 1.0;
                }
                //draws graph
                for (index, interval) in speed_stat_scaled.into_iter().enumerate() {
                    ctx.draw(&Rectangle {
                        x: index as f64 * (width / len as f64),
                        y: 0.0,
                        width: width / len as f64,
                        height: (height / max_interval as f64) * interval as f64,
                        color: Color::White,
                    });
                }
            });

        f.render_widget(canvas, graph_zone)
    }
}

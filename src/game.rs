use std::time::{Duration, SystemTime};

use markov_rope::MarkovChain;

#[derive(Clone, Debug, PartialEq)]
pub enum GameMode {
    Normal,
    Rewrite,
}
#[derive(Clone, Debug)]
pub struct GameConf {
    pub mode: GameMode,
    pub limit: Limit,
}
impl GameConf {
    pub fn new() -> GameConf {
        GameConf {
            mode: GameMode::Normal,
            limit: Limit::Time(Duration::from_secs(30)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Limit {
    Time(Duration),
    WordCount(u32),
    None,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub correct_text: Vec<char>,
    pub written_vec: Vec<Letter>,
    pub statistics: GameStat,
    // pub game_mode: GameMode,
    pub game_conf: GameConf,
}
impl Game {
    pub fn new(size: usize, conf: GameConf, text: Option<String>) -> Game {
        Game {
            correct_text: {
                let text = if let Some(t) = text {
                    t.clone()
                }else{
                    "".to_string()
                };
                let mr = if text.len() > 0{
                    MarkovChain::from_string(&text)
                }else{
                    MarkovChain::default()
                };
                
                if let Limit::WordCount(t) = conf.limit {
                    //generate new text if limit is word count
                    mr.generate_text(t + 1).unwrap().chars().collect()//? dangerous unwrapping
                } else {
                    mr.generate_text(size as u32).unwrap().chars().collect() //? dangerous unwrapping
                }
            },

            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: mode,
            game_conf: conf,
        }
    }

    /// "Press" char key for written text
    pub fn char_key_pressed(&mut self, c: char) {
        if self.correct_text[self.written_vec.len()] == c {
            self.written_vec.push(Letter {
                c,
                state: FieldState::Correct,
            });
            self.statistics.correct_strokes += 1;
        } else {
            self.written_vec.push(Letter {
                c: self.correct_text[self.written_vec.len()].clone(),
                state: FieldState::Wrong,
            });
            self.statistics.wrong_strokes += 1;
        }

        //section for speed tracking (probably not very optimized)

        if c != ' ' {
            return; //checking if it is the end of potential word
        }
    }

    /// Returns true if length of written text is the same as length of correct text
    pub fn is_complete(&self) -> bool {
        if self.written_vec.len() == self.correct_text.len() {
            return true;
        } //needed even for time limit

        match self.game_conf.limit {
            Limit::Time(t) => {
                let time_passed = SystemTime::now()
                    .duration_since(self.statistics.time_started)
                    .unwrap(); //?Probably not safe
                time_passed >= t
            }
            Limit::WordCount(count) => self.get_total_words_count() >= count, //?Possible problem if statistics are not updated, should update statistic after every input
            Limit::None => false,
        }
    }

    ///does everything needed to be done after the game ends
    pub fn stop_game(&mut self) {
        self.register_speed();
    }

    /// "Press" backspace for written text, deletes 1 correct stroke if letter is correct
    pub fn clear_last_letter(&mut self) {
        let letter = self.written_vec.pop();
        if letter.is_some() && letter.unwrap().state == FieldState::Correct {
            self.statistics.correct_strokes -= 1; //needed to prevent abusive deleting and placing same letters for higher accuracy
        }
    }
    pub fn clear_last_world(&mut self) {
        loop {
            self.clear_last_letter();
            if self
                .written_vec
                .last()
                .is_some_and(|l| l.c == ' ' && l.state == FieldState::Correct)
            {
                break;
            }
            if !self.written_vec.last().is_some() {
                break; //stop deleting letters if no letters left
            }
        }
    }

    /// Gets vector of letters including unfilled letters from correct text
    /// Used only for rendering
    ///
    ///
    pub fn get_written_vec(&mut self) -> Vec<Letter> {
        let mut res = self.written_vec.clone();
        for c in &self.correct_text[self.written_vec.len()..] {
            res.push(Letter {
                c: c.clone(),
                state: FieldState::Unfilled,
            });
        }
        res
    }

    ///Gets random text for a game
    // pub fn get_random_text(size: usize) -> String {
    //     //crate::markov_gen::generate(size)
    //     let mr = markov_rope::MarkovChain::default();
    //     mr.generate_text(size as u32).unwrap() ////!TESTING
    // }

    ///Writes to speed stat vector
    pub fn register_speed(&mut self) {
        self.statistics
            .speed_stat
            .push(self.get_correct_words_count() - self.statistics.speed_stat.iter().sum::<u32>())
    }

    /// Returns game statistics
    pub fn get_statistics(&mut self) -> GameStat {
        self.statistics = GameStat {
            correct_strokes: self.statistics.correct_strokes.clone(),
            wrong_strokes: self.statistics.wrong_strokes.clone(),
            wrong_letters: self
                .written_vec
                .iter()
                .filter(|letter| letter.state == FieldState::Wrong)
                .count() as u32,
            correct_words: self.get_correct_words_count(),
            speed_stat: self.statistics.speed_stat.clone(),
            total_words: self.get_total_words_count(),
            time_started: self.statistics.time_started.clone(),
            time_finished: SystemTime::now(),
            intervals: 0,
        };
        self.statistics.clone()
    }
    pub fn get_correct_words_count(&self) -> u32 {
        let mut count = 0;
        let mut is_wrong = false;
        for l in &self.written_vec {
            if l.state != FieldState::Correct {
                is_wrong = true;
            }
            if l.c == ' ' {
                if !is_wrong {
                    count += 1;
                }
                is_wrong = false;
            }
        }
        count
    }
    pub fn get_total_words_count(&self) -> u32 {
        let len = self.written_vec.len();

        let count = self.correct_text[..len]
            .iter()
            .filter(|c| **c == ' ')
            .count() as u32;
        // if self.correct_text.len() > 0 {
        //     return count + 1;//?Needed for last word to count but brakes word limit
        // }
        count
    }
}
#[derive(Clone, Debug)]
pub struct GameStat {
    pub correct_strokes: u32,
    pub wrong_strokes: u32,
    pub wrong_letters: u32,
    pub correct_words: u32,
    ///first is vector second is interval
    pub speed_stat: Vec<u32>,
    pub total_words: u32,
    pub time_started: SystemTime,
    pub time_finished: SystemTime,
    pub intervals: i32,
}
impl GameStat {
    pub fn new() -> GameStat {
        GameStat {
            correct_strokes: 0,
            wrong_strokes: 0,
            wrong_letters: 0,
            correct_words: 0,
            speed_stat: vec![],
            total_words: 0,
            time_started: SystemTime::now(),
            time_finished: SystemTime::now(),
            intervals: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Letter {
    pub c: char,
    pub state: FieldState,
}
#[derive(PartialEq, Debug, Clone)]
pub enum FieldState {
    Unfilled,
    Correct,
    Wrong,
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use super::*;
    #[test]
    fn match_all_correct() {
        let mut test_game = Game {
            correct_text: "ccccc".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: GameMode::Normal,
            game_conf: GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
        };
        let written_text: Vec<char> = "ccccc".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }

    #[test]
    fn match_all_wrong() {
        let mut test_game = Game {
            correct_text: "wwwww".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: GameMode::Normal,
            game_conf: GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
        };
        let written_text: Vec<char> = "-----".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }

    #[test]
    fn match_mixed() {
        let mut test_game = Game {
            correct_text: "ccwcu".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: GameMode::Normal,
            game_conf: GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
        };
        let written_text: Vec<char> = "cc-c".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'w',
                state: FieldState::Wrong,
            },
            Letter {
                c: 'c',
                state: FieldState::Correct,
            },
            Letter {
                c: 'u',
                state: FieldState::Unfilled,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }

    #[test]
    fn get_correct_words_test() {
        let mut test_game = Game {
            correct_text: "cc cc cc cc".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: GameMode::Normal,
            game_conf: GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
        };
        let written_text: Vec<char> = "ccuuc cc cc".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        assert_eq!(test_game.get_correct_words_count(), 2);
    }

    #[test]
    fn get_total_words_test() {
        let mut test_game = Game {
            correct_text: "cc cc cc cc".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
            // game_mode: GameMode::Normal,
            game_conf: GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
        };
        let written_text: Vec<char> = "ccuuc cc cc".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        assert_eq!(test_game.get_total_words_count(), 4);
    }

    #[ignore = "makes thread sleepy -_- zzz"]
    #[test]
    fn get_time_test() {
        let mut test_game = Game::new(
            90,
            GameConf {
                mode: GameMode::Normal,
                limit: Limit::None,
            },
            None,
        );
        let pass_dur = Duration::new(2, 0);
        sleep(pass_dur);

        let stats = test_game.get_statistics();
        let time_passed = stats
            .time_finished
            .duration_since(stats.time_started)
            .expect("error calculating time thing idk");
        assert!(time_passed.checked_sub(pass_dur).unwrap() < Duration::from_secs_f32(0.001));
    }
}

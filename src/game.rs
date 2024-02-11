use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Game {
    pub correct_text: Vec<char>,
    pub written_vec: Vec<Letter>,
    pub statistics: GameStat,
}
impl Game {
    pub fn new() -> Game {
        Game {
            correct_text: Game::get_random_text(5).chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
        }
    }

    /// "Press" char key for written text
    pub fn char_key_pressed(&mut self, c: char) {
        if self.correct_text[self.written_vec.len()] == c {
            self.written_vec.push(Letter {
                c,
                state: FieldState::Correct,
            });
        } else {
            self.written_vec.push(Letter {
                c: self.correct_text[self.written_vec.len()].clone(),
                state: FieldState::Wrong,
            });
        }
    }

    /// Returns true if length of written text is the same as length of correct text
    pub fn is_complete(&self) -> bool {
        self.written_vec.len() == self.correct_text.len()
    }

    /// "Press" backspace for written text, doesn't allow to delete letters of correctly finished word
    pub fn backspace_pressed(&mut self) {
        self.written_vec.pop();
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
    pub fn get_random_text(size: usize) -> String {
        //String::from("Ryan Thomas Gosling is a Canadian actor. Prominent in both independent film and major studio features of varying genres, his films have accrued a worldwide box office gross of over 1.9 billion USD. He has received various accolades, including a Golden Globe Award, as well as nominations for three Academy Awards and two BAFTAs. ")
        let mut chain = markov::Chain::new();
        let er = chain.feed_file("text_for_markov.txt");
        match er {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }
        let a = &chain.generate_str()[..size];
        String::from(a)
    }

    /// Returns game statistics
    pub fn get_statistics(&mut self) -> GameStat {
        self.statistics = GameStat {
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
        if !is_wrong {
            count += 1; //crunch to fix las word check, //TODO: Fix later
        }
        count
    }
    pub fn get_total_words_count(&self) -> u32 {
        let len = self.written_vec.len();

        let count = self.correct_text[..len]
            .iter()
            .filter(|c| **c == ' ')
            .count() as u32;
        if self.correct_text.len() > 0 {
            return count + 1;
        }
        count
    }
}
#[derive(Clone, Debug)]
pub struct GameStat {
    wrong_letters: u32,
    correct_words: u32,
    speed_stat: Vec<u32>,
    total_words: u32,
    time_started: SystemTime,
    time_finished: SystemTime,
}
impl GameStat {
    pub fn new() -> GameStat {
        GameStat {
            wrong_letters: 0,
            correct_words: 0,
            speed_stat: vec![],
            total_words: 0,
            time_started: SystemTime::now(),
            time_finished: SystemTime::now(),
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
    use std::{os::linux::raw::stat, thread::sleep, time::Duration};

    use super::*;
    #[test]
    fn match_all_correct() {
        let mut test_game = Game {
            correct_text: "ccccc".chars().collect(),
            written_vec: Vec::new(),
            statistics: GameStat::new(),
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
        let mut test_game = Game::new();
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

#[derive(Clone, Debug)]
pub struct Game {
    pub correct_text: Vec<char>,
    pub written_vec: Vec<Letter>,
}
impl Game {
    pub fn new() -> Game {
        Game {
            correct_text: Game::get_random_text(5).chars().collect(),
            written_vec: Vec::new(),
        }
    }

    /// "Press" char key for written text
    pub fn char_key_pressed(&mut self, c: char) {
        if self.correct_text[self.written_vec.len()] == c {
            self.written_vec.push(Letter {
                c,
                state: LetterState::Correct,
            });
        } else {
            self.written_vec.push(Letter {
                c: self.correct_text[self.written_vec.len()].clone(),
                state: LetterState::Wrong,
            });
        }
    }

    /// Returns true if length of written text is the same as length of correct text
    pub fn is_complete(&self) -> bool {
        self.written_vec.len() == self.correct_text.len()
    }

    /// "Press" backspace for written text
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
                state: LetterState::Unfilled,
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Letter {
    pub c: char,
    pub state: LetterState,
}
#[derive(PartialEq, Debug, Clone)]
pub enum LetterState {
    Unfilled,
    Correct,
    Wrong,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn match_all_correct() {
        let mut test_game = Game {
            correct_text: "ccccc".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "ccccc".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }

    #[test]
    fn match_all_wrong() {
        let mut test_game = Game {
            correct_text: "wwwww".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "-----".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }

    #[test]
    fn match_mixed() {
        let mut test_game = Game {
            correct_text: "ccwcu".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "cc-c".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'w',
                state: LetterState::Wrong,
            },
            Letter {
                c: 'c',
                state: LetterState::Correct,
            },
            Letter {
                c: 'u',
                state: LetterState::Unfilled,
            },
        ];
        assert_eq!(test_game.get_written_vec(), answer);
    }
}

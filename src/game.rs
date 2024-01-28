#[derive(Clone)]
#[derive(Debug)]
pub struct Game {
    pub correct_text: Vec<char>,
    pub written_vec: Vec<Letter>,
}
impl Game {
    pub fn new() -> Game{
        Game {
            correct_text: Game::get_random_test().chars().collect(),
            written_vec: Vec::new(),
        }
    }
    pub fn char_key_pressed(&mut self,c:char){
        if self.correct_text[self.written_vec.len()] == c {
            self.written_vec.push(Letter { c, state: LetterState::Correct });
        } else {
            self.written_vec.push(Letter { c: self.correct_text[self.written_vec.len()].clone(), state: LetterState::Wrong });
        }
    }

    pub fn backspace_pressed(&mut self){
        self.written_vec.pop();
    }

    pub fn get_written_vec(&mut self) -> Vec<Letter>{
        let mut res = self.written_vec.clone();
        for c in &self.correct_text[self.written_vec.len()..]{
            res.push(Letter{ c:c.clone(), state: LetterState::Unfilled });
        }
        res
    }

    pub fn get_random_test() -> String {
        String::from("Not a random text, only used for testing")
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Letter{
    pub c: char,
    pub state: LetterState
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum LetterState {
    Unfilled,
    Correct,
    Wrong
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn match_all_correct(){
        let mut test_game = Game{
            correct_text: "ccccc".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "ccccc".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},];
        assert_eq!(test_game.get_written_vec(),answer);
    }

    #[test]
    fn match_all_wrong(){
        let mut test_game = Game{
            correct_text: "wwwww".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "-----".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},];
        assert_eq!(test_game.get_written_vec(),answer);
    }

    #[test]
    fn match_mixed(){
        let mut test_game = Game{
            correct_text: "ccwcu".chars().collect(),
            written_vec: Vec::new(),
        };
        let written_text: Vec<char> = "cc-c".chars().collect();
        for c in written_text {
            test_game.char_key_pressed(c);
        }
        let answer = vec![Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'w',state: LetterState::Wrong},Letter{c:'c',state: LetterState::Correct},Letter{c:'u',state: LetterState::Unfilled},];
        assert_eq!(test_game.get_written_vec(),answer);
    }
    
}
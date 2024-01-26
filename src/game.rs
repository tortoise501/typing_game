use crate::component::*;
use crossterm::event::KeyCode;
#[allow(dead_code)]
#[allow(unused_variables)]
struct Game {
    correct_text: Vec<char>,
    written_vec: Vec<Letter>,
}
impl Component for Game {
    fn init(&mut self) -> std::io::Result<()> {
        Ok(())   
    }
    fn handle_key_events(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char(c) => {
                self.char_key_pressed(c);
            },
            KeyCode::Backspace => {
                self.backspace_pressed();
            },
            _ => ()
        }
        Action::Noop
    }
}



#[allow(dead_code)]
#[allow(unused_variables)]
impl Game {
    pub fn new() -> Game{
        Game {
            correct_text: Game::get_random_test().chars().collect(),
            written_vec: Vec::new(),
        }
    }
    pub fn char_key_pressed(&mut self,c:char){
        if self.correct_text[self.written_vec.len()] == c {
            self.written_vec.push(Letter{ c, state: LetterState::Correct });
        } else {
            self.written_vec.push(Letter{ c: self.correct_text[self.written_vec.len()].clone(), state: LetterState::Wrong });
        }
    }

    pub fn backspace_pressed(&mut self){
        self.written_vec.pop();
    }

    pub fn get_written_vec(&mut self) -> Vec<Letter>{
        let mut res = self.written_vec.clone();
        for c in &self.correct_text[self.written_vec.len()..]{
            println!("test:{}",c);
            res.push(Letter{ c:c.clone(), state: LetterState::Unfilled });
        }
        res
    }

    pub fn get_matched_letter_vec(&self,correct_string: &String, written_string: &String) -> &Vec<Letter>{
        // let mut res: Vec<Letter> = Vec::new();
        // let correct_string: Vec<char> = correct_string.chars().collect();
        // let written_string: Vec<char> = written_string.chars().collect();
    
        // if written_string.len() > correct_string.len(){
        //     panic!("written text bigger than correct text");
        // }
    
        
    
    
        // for (i,c) in correct_string.iter().enumerate(){
        //     if i >= written_string.len(){
        //         res.push(Letter {c:c.clone(),state:LetterState::Unfilled});
        //         continue;
        //     }
        //     let letter_state: LetterState = match c {
        //         c if c == &written_string[i] => LetterState::Correct,
        //         _ => LetterState::Wrong,
        //     };
        //     res.push(Letter {c:c.clone(),state:letter_state});
        // }
    
        // res
        &self.written_vec
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
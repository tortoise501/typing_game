

extern crate crossterm;

mod gameio;

pub fn start(){
    println!("start test");
    start_game();
}

fn start_game(){
    let created_text = get_random_test();
    let mut written_text = String::new();
    loop{
        match gameio::input::read_key(){
            None => (),
            Some(c) => {
                if c == '\u{232B}' {
                    written_text.pop();
                }
                else {
                    written_text.push_str(c.to_string().as_str())
                }
            },
        }
        let matched_text = get_matched_letter_vec(&created_text,&written_text);
        match gameio::output::print_game_text(&matched_text) {
            Ok(msg) => _ = msg,
            Err(msg) => panic!("Error in output function, {}",msg)
        }

    }

}



fn get_random_test() -> String {
    String::from("Not a random text, only used for testing")
}

fn get_matched_letter_vec(correct_string: &String, written_string: &String) -> Vec<Letter>{
    let mut res: Vec<Letter> = Vec::new();
    let correct_string: Vec<char> = correct_string.chars().collect();
    let written_string: Vec<char> = written_string.chars().collect();

    if written_string.len() > correct_string.len(){
        panic!("written text bigger than correct text");
    }

    


    for (i,c) in correct_string.iter().enumerate(){
        if i >= written_string.len(){
            res.push(Letter {c:c.clone(),state:LetterState::Unfilled});
            continue;
        }
        let letter_state: LetterState = match c {
            c if c == &written_string[i] => LetterState::Correct,
            _ => LetterState::Wrong,
        };
        res.push(Letter {c:c.clone(),state:letter_state});
    }

    res
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Letter{
    c: char,
    state: LetterState
}
#[derive(PartialEq)]
#[derive(Debug)]
enum LetterState {
    Unfilled,
    Correct,
    Wrong
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn match_all_correct(){
        let test = get_matched_letter_vec(&String::from("ccccc"), &String::from("ccccc"));
        let answer = vec![Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},];
        assert_eq!(test,answer);
    }
    #[test]
    fn match_all_wrong(){
        let test = get_matched_letter_vec(
            &String::from("wwwww"), 
            &String::from("-----")
        );
        let answer = vec![Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},Letter{c:'w',state: LetterState::Wrong},];
        assert_eq!(test,answer);
    }
    #[test]
    fn match_mixed(){
        let test = get_matched_letter_vec(
            &String::from("ccwcu"), 
            &String::from("cc-c")
        );
        let answer = vec![Letter{c:'c',state: LetterState::Correct},Letter{c:'c',state: LetterState::Correct},Letter{c:'w',state: LetterState::Wrong},Letter{c:'c',state: LetterState::Correct},Letter{c:'u',state: LetterState::Unfilled},];
        assert_eq!(test,answer);
    }
    
}
extern crate crossterm;

mod gameio;






pub fn start(){
    println!("tests");
    start_game();
}

fn start_game(){
    let created_text = get_random_test();
    let mut written_text = String::new();
    loop{
        match gameio::input::read_key(){
            None => (),
            Some(c) => written_text.push_str(c.to_string().as_str()),
        }
        let matched_text = get_matched_strings(&written_text, &created_text);
        match gameio::output::print_game_text(&matched_text) {
            Ok(msg) => _ = msg,
            Err(msg) => panic!("Error in output function, {}",msg)
        }

    }

}



fn get_random_test() -> String {
    String::from("Not a random text, only used for testing")
}

fn get_matched_strings(written_text:&String, created_text:&String) -> Vec<TypedString> {
    let written_text:Vec<char> = written_text.chars().collect();
    let created_text:Vec<char> = created_text.chars().collect();

    let mut res:Vec<TypedString> = Vec::new();

    let mut last_text_status = TextStatus::Unfilled;
    let mut one_status_string = String::new();
    
    let mut push_new_status_string = |c: &char, status: TextStatus, new: bool| {
        if new{
            let test = TypedString{
                text_status: status,
                text: one_status_string.clone(),
            };
            res.push(test);
            one_status_string = String::new();
        }
        one_status_string.push(c.clone());
    };

    let mut compare_for = |c: &char, status:TextStatus|{
        match last_text_status as i32 * status as i32 {
            1 => {
                push_new_status_string(c,status,false)
            },
            -1  => {
                push_new_status_string(c,status,true);
                last_text_status = status;
            },
            0 => {
                push_new_status_string(c,TextStatus::Filled,false);
                last_text_status = status
            },
            _ => todo!()
        }
    };

    for (i, c1) in written_text.iter().enumerate(){
        let c2 = &created_text[i];
        if c1 == c2 {
            compare_for(c2,TextStatus::Filled);
        }else{
            compare_for(c2,TextStatus::Wrong);
        }
    }
    res.push(TypedString{
        text_status: last_text_status,
        text: one_status_string.clone(),
    });
    res
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn matching_fully(){
        let test = get_matched_strings(&String::from("the cake is a lie"), &String::from("the cake is a lie"));
        assert_eq!(test,vec![TypedString{text_status:TextStatus::Filled,text:String::from("the cake is a lie")}]);
    }
    #[test]
    fn not_matching_fully(){
        let test = get_matched_strings(&String::from("the cake is a lie"), &String::from("i like pierogi and herbata"));
        assert_eq!(test,vec![TypedString{text_status:TextStatus::Wrong,text:String::from("i like pierogi an")}]);
    }
}



#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum TextStatus{
    Unfilled = 0,
    Filled = 1,
    Wrong = -1
}
impl Copy for TextStatus {
}
#[derive(PartialEq)]
#[derive(Debug)]
struct TypedString{
    text_status: TextStatus,
    text: String,
}
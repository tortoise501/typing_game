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

    let mut last_text_status = None;
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
        

        let last_text_status_some = match last_text_status {
            Some(st) => st,
            None => TextStatus::Unfilled,
        };
        match last_text_status_some as i32 * status as i32 {
            1 => {
                push_new_status_string(c,last_text_status_some,false)
            },
            -1  => {
                push_new_status_string(c,last_text_status_some,true);
                last_text_status = Some(status);
            },
            0 => {
                let do_new = match last_text_status_some {
                    TextStatus::Unfilled => false,
                    _ => true
                };
                push_new_status_string(c,last_text_status_some,do_new);
                last_text_status = Some(status)
            },
            _ => todo!()
        }
    };

    for (i, c1) in created_text.iter().enumerate(){
        if written_text.len() <= i{
            compare_for(c1,TextStatus::Unfilled);
            continue;
        }
        let c2 = &written_text[i];
        if c1 == c2 {
            compare_for(c1,TextStatus::Filled);
        }else{
            compare_for(c1,TextStatus::Wrong);
        }
    }
    res.push(TypedString{
        text_status: match last_text_status {
            Some(st) => st,
            None => TextStatus::Unfilled,
        },
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
        let test = get_matched_strings(
            &String::from("the many shall suffer for the sins of the one"),
            &String::from("i like pierogis and herbata,-----------------"));
        assert_eq!(test,vec![TypedString{text_status:TextStatus::Wrong,text:String::from("i like pierogis and herbata,-----------------")},]);
    }
    #[test]
    fn matching_not_fully(){
        let test = get_matched_strings(
            &String::from("This is filled string,wrong chars only, but this is"), 
            &String::from("This is filled string, and this isn't , but this is,this is unfilled")
        );
        assert_eq!(
            test,
            vec![
                TypedString{text_status:TextStatus::Filled,text:String::from("This is filled string,")},
                TypedString{text_status:TextStatus::Wrong,text:String::from(" and this isn't ")},
                TypedString{text_status:TextStatus::Filled,text:String::from(", but this is")},
                TypedString{text_status:TextStatus::Unfilled,text:String::from(",this is unfilled")},
                ]
        );
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
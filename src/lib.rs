use crossterm::event::KeyCode;



extern crate crossterm;

mod gameio;
mod game;
mod component;

pub fn start(){
    println!("start test");
    start_game();
}

fn start_game(){
    // let created_text = get_random_test();
    // let mut written_text = String::new();
    // loop{
    //     match gameio::input::read_key(){
    //         None => (),
    //         Some(c) => {
    //             if c == '\u{232B}' {
    //                 written_text.pop();
    //             }
    //             else {
    //                 written_text.push_str(c.to_string().as_str())
    //             }
    //         },
    //     }
    //     let matched_text = get_matched_letter_vec(&created_text,&written_text);
    //     match gameio::output::print_game_text(&matched_text) {
    //         Ok(msg) => _ = msg,
    //         Err(msg) => panic!("Error in output function, {}",msg)
    //     }

    // }

}
















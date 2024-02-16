use markov_strings::*;
use std::fs;
pub fn generate(len: usize) -> String {
    let mut markov = Markov::new();

    // Optional: specify a state size
    markov.set_state_size(1).expect("idk error"); // Default: 2

    // Feed it data
    let data: Vec<InputData> = vec![fs::read_to_string("text_for_markov.txt")
        .expect("Should have been able to read the file")
        .to_string()]
    .iter()
    .map(|s| s.to_owned().into())
    .collect();
    markov.add_to_corpus(data);

    let mut res = String::new();

    while res.as_str().split(' ').count() < len {
        res.push_str(markov.generate().expect("err").text.as_str());
    }
    res.as_str()
        .split(' ')
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()[..len]
        .join(" ")
}

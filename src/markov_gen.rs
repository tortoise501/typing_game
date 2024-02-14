use markov_strings::*;
use std::fs;
pub fn generate(len: usize) -> String {
    let mut markov = Markov::new();

    // Optional: specify a state size
    markov.set_state_size(1); // Default: 2

    // Feed it data
    let data: Vec<InputData> = vec![fs::read_to_string("text_for_markov.txt")
        .expect("Should have been able to read the file")
        .to_string()]
    .iter()
    .map(|s| s.to_owned().into())
    .collect();
    markov.add_to_corpus(data);

    // Define a results filter
    // markov
    //     .set_filter(|r| {
    //         // A minimal relative score and number of references
    //         // The thresholds are relative to your input
    //         r.score > 5 && r.refs.len() > 2
    //         // We want to generate random tweets
    //         && r.text.len() <= 280000
    //         // No mentions
    //         && !r.text.contains("@")
    //         // No urls
    //         && !r.text.contains("http")
    //     })
    //     .set_max_tries(100);

    let result: MarkovResult = markov.generate().expect("err");
    result
        .text
        .as_str()
        .split(' ')
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

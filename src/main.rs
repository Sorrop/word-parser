use std::env;
use std::str;
mod finder;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <string> ");
        return;
    }

    let out = parse_words(&args[1]);
    output(out);

}

fn parse_words (s: &String) -> Vec<&str> {

    let mut words: Vec<&str> = Vec::new();

    //i will be a cursor that will hold the index to the first character of the next word
    let mut i = 0;
    let size = s.len();
    let raw_bytes = s.as_bytes();

    while i < size {
        let word_byte = finder::first_word(&raw_bytes[i..size]);
        let word = str::from_utf8(word_byte).unwrap();
        words.push(word);
        i = i + word.len() + 1;
    }
    return words;
}

fn output(words: Vec<&str>) {
    for (j, ref word) in words.iter().enumerate() {
        println!("{},{},{}", j+1, word, word.len());
    }
}

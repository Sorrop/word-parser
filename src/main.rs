use std::env;
use std::str;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <string> ");
        return;
    }

    let out = parse_words(&args[1]);
    println!("Out: {}", out.len());

}

fn parse_words (s: &String) -> Vec<String> {

    let mut words: Vec<String> = Vec::new();

    //i will be a cursor that will hold the index to the first character of the next word
    let mut i = 0;
    let size = s.len();
    let raw_bytes = s.as_bytes();

    while i < size {
        i = i + 1;
    }
    return words;
}

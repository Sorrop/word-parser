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
    return words;
}

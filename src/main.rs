use std::env;
use std::fs::File;
use std::io::prelude::*;
mod finder;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <filename> ");
        return;
    }

    let file_contents = slurp_file(&args[1]);

    let out = finder::parse_words(&file_contents);
    output(out);

}

fn slurp_file(filename: &String) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents
}



fn output(words: Vec<&str>) {
    for (j, ref word) in words.iter().enumerate() {
        println!("{},{},{}", j+1, word, word.len());
    }
}

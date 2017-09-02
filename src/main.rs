use std::env;
mod finder;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <string> ");
        return;
    }

    let out = finder::parse_words(&args[1]);
    output(out);

}



fn output(words: Vec<&str>) {
    for (j, ref word) in words.iter().enumerate() {
        println!("{},{},{}", j+1, word, word.len());
    }
}

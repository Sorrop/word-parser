use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <string> ");
        return;
    }
}

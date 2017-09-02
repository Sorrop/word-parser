use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
mod finder;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!(" Usage: cargo run <input-file> <output-file>");
        return;
    }

    let file_contents = slurp_file(&args[1]);

    let out = finder::parse_words(&file_contents);
    output(out, &args[2]);

}

fn slurp_file(filename: &String) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents
}



fn output(words: Vec<&str>, filename: &str) {

    let outfile = File::create(filename);
    let mut bufwriter = BufWriter::new(outfile.unwrap());
    for (j, ref word) in words.iter().enumerate() {
        let _ = writeln!(& mut bufwriter, "{},{},{}", j+1, word, word.len());
    }
    return;
}

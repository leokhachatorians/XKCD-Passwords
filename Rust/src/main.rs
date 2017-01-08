extern crate docopt;
extern crate rustc_serialize;
extern crate rand;

use docopt::Docopt;
use rand::Rng;
use std::{process};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const USAGE: &'static str = "
Usage:
    genxkcd-pass [options] [-n <number>] [-p <path>]

Options:
    -h --help
    -n --number <number>                        Number of words to generate [default: 5].
    -w --word-list <path/to/wordlist.txt>       Override default wordlist [default: ../wordlist.txt].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_h: bool,   // help menu
    flag_n: i32,    // number of words
    flag_w: String, // any external wordlist
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_n < 1 {
        println!("Error: Word count must be greater than 0");
        process::exit(1);
    }

    let f = match File::open(args.flag_w) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let mut word_vec: Vec<String> = Vec::new();
    let reader = BufReader::new(f);
    let lines: Result<Vec<_>, _> = reader.lines().collect();

    for word in lines.unwrap() {
        word_vec.push(word);
    }

    for _ in 0..args.flag_n {
        let num = rand::thread_rng().gen_range(0, word_vec.len());
        print!("{} ", word_vec[num]);
    }

}

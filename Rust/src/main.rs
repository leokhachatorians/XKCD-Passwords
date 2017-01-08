extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::{process};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
//use std::vec::Vec;

static DEFAULT_WORDLIST: &'static str = include_str!("../../wordlist.txt");
const USAGE: &'static str = "
Usage:
    genxkcd-pass [options] [-n <number>] [-p <path>]

Options:
    -h --help
    -n --number <number>                Number of words to generate [default: 5].
    -p --path <path/to/wordlist.txt>    Override default wordlist [default: DEFAULT_WORDLIST].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_h: bool,   // help menu
    flag_n: i32,    // number of words
    flag_p: String, // any external wordlist
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_n < 1 {
        println!("Error: Word count must be greater than 0");
        process::exit(1);
    }

    println!("{:?}", args);

    let f = match File::open("../../wordlist.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("{}",e);
            process::exit(1);
        }
    };

    let mut word_vec: Vec<String> = Vec::new();
    let reader = BufReader::new(f);
    let lines: Result<Vec<_>, _> = reader.lines().collect();

    for word in lines.unwrap() {
        word_vec.push(word);
    }

    for i in 0..args.flag_n as usize {
        println!("{}", word_vec[i]);
    }

}

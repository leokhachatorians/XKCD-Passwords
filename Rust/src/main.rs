extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::{process};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
//use std::vec::Vec;

static DEFAULT_WORDLIST: &'static str = include_str!("words.txt");
const USAGE: &'static str = "
Usage:
    genxkcd-pass [options] [-n <number>] [-p <path>]

Options:
    -h --help
    -n --number <number>                Number of words to generate
    -p --path <path/to/wordlist.txt>    Override default wordlist
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
    println!("{}", DEFAULT_WORDLIST);

    let mut v: Vec<String> = Vec::new();

    let f = File::open("words.txt").unwrap()

    //let f = read_file();
    //
    
   // let f = match File::open("words.txt") {
   //     Ok(file) => file,
   //     Err(e) => {
   //         println!("{}",e);
   //         process::exit(1);
   //     }
   // };


    //let lists = vec![DEFAULT_WORDLIST.to_string()];


    //let mut reader = BufReader::new(f);

   // for line in reader.lines() {
   //     v.push(line);
   // }
}

fn read_file() -> Result<(), io::Error> {
    let f = try!(File::open(DEFAULT_WORDLIST));
    Ok(())
}

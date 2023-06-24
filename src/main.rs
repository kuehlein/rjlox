mod scanner;
mod token;
pub mod utils;

#[macro_use]
extern crate phf;

use std::env;
use std::fs;
use std::io::{self, BufRead, Error};
use std::process;

use scanner::Scanner;

mod prelude {
    // crates
    use std::sync::Mutex;

    // internal modules
    pub use crate::utils;

    // static variables
    pub static HAD_ERROR: Mutex<bool> = Mutex::new(false);
}

fn run(source: Result<String, Error>) {
    match source {
        Ok(src) => {
            let mut scanner = Scanner::new(src);
            let tokens = scanner.scan_tokens();

            tokens.iter().for_each(|token| println!("{}", token));
        },
        Err(error) => panic!("Failed to read source code: {}", error),
    }
}

/// run interpreter interactively - enter & execute one line of code at a time
fn run_prompt() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        run(line);
    }
}

fn run_file(path: String) {
    let file_contents = fs::read_to_string(path);

    run(file_contents);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(args.pop().unwrap()),
        _ => {
            eprintln!("Usage: rjlox [script]");
            process::exit(64);
        }
    }
}

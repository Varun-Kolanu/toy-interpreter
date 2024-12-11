mod ast;
mod scanner;
mod token;
mod utils;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            run_file(filename);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn run_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    run(file_contents);
}

fn run(source: String) {
    let mut scanner = Scanner {
        source,
        ..Default::default()
    };
    scanner.scan_tokens();

    for token in scanner.tokens {
        println!("{}", token.to_string());
    }

    if scanner.had_error {
        exit(65);
    }
}

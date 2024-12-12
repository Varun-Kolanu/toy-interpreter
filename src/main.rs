mod ast;
mod ast_printer;
mod scanner;
mod token;
mod utils;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use ast::Expr;
use ast_printer::AstPrinter;
use scanner::Scanner;
use token::{LiteralType, Token, TokenType};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match command.as_str() {
        "tokenize" => {
            let filename = &args[2];
            run_file(filename);
            if args.len() < 3 {
                writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
                return;
            }
        }
        "print_ast" => {
            let expression = Box::new(Expr::Binary {
                left: Box::new(Expr::Unary {
                    operator: Token {
                        token_type: TokenType::MINUS,
                        lexeme: String::from("-"),
                        literal: LiteralType::NULL,
                        line: 1,
                    },
                    right: Box::new(Expr::Literal {
                        value: LiteralType::NUMBER(123.0),
                    }),
                }),
                operator: Token {
                    token_type: TokenType::STAR,
                    lexeme: String::from("*"),
                    literal: LiteralType::NULL,
                    line: 1,
                },
                right: Box::new(Expr::Grouping {
                    expression: Box::new(Expr::Literal {
                        value: LiteralType::NUMBER(45.67),
                    }),
                }),
            });
            println!("{}", AstPrinter {}.print(&expression))
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

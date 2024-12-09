use std::env;
use std::fs;
use std::io::{self, Write};

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
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: String::from("null"),
            line: self.line,
        })
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, String::from("null")),
            ')' => self.add_token(TokenType::RIGHT_PAREN, String::from("null")),
            '{' => self.add_token(TokenType::LEFT_BRACE, String::from("null")),
            '}' => self.add_token(TokenType::RIGHT_BRACE, String::from("null")),
            _ => error(
                self.line,
                String::from(format!("Unexpected character {}", c)),
            ),
        }
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap_or_else(|| {
            writeln!(
                io::stderr(),
                "Index out of bounds for source at {}",
                self.current
            )
            .unwrap();
            return '\0';
        });
        self.current += 1;
        return character;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
            literal,
        });
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            source: String::from(""),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    EOF,
}

#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

fn error(line: usize, message: String) {
    writeln!(io::stderr(), "[line {}] Error: {}", line, message).unwrap();
}

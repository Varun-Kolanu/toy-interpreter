use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
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
        });
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_non_literal_token(TokenType::LEFT_PAREN),
            ')' => self.add_non_literal_token(TokenType::RIGHT_PAREN),
            '{' => self.add_non_literal_token(TokenType::LEFT_BRACE),
            '}' => self.add_non_literal_token(TokenType::RIGHT_BRACE),
            ',' => self.add_non_literal_token(TokenType::COMMA),
            '.' => self.add_non_literal_token(TokenType::DOT),
            '-' => self.add_non_literal_token(TokenType::MINUS),
            '+' => self.add_non_literal_token(TokenType::PLUS),
            ';' => self.add_non_literal_token(TokenType::SEMICOLON),
            '*' => self.add_non_literal_token(TokenType::STAR),
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_non_literal_token(token_type);
            }
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_non_literal_token(token_type);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_non_literal_token(token_type);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_non_literal_token(token_type);
            }
            '/' => {
                if self.match_next('/') {
                    while !self.is_at_end() && get_char(&self.source, self.current) != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_non_literal_token(TokenType::SLASH);
                }
            }

            _ => {
                self.had_error = true;
                error(
                    self.line,
                    String::from(format!("Unexpected character: {}", c)),
                )
            }
        }
    }

    fn advance(&mut self) -> char {
        let character = get_char(&self.source, self.current);
        self.current += 1;
        return character;
    }

    fn match_next(&mut self, character_to_match: char) -> bool {
        let current_char = get_char(&self.source, self.current);
        if current_char != character_to_match {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn add_non_literal_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, String::from("null"));
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
            had_error: false,
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
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,

    // One or two character tokens
    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    SLASH,

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

fn get_char(text: &str, index: usize) -> char {
    return text.chars().nth(index).unwrap_or_else(|| {
        writeln!(io::stderr(), "Index out of bounds for source at {}", index).unwrap();
        return '\0';
    });
}

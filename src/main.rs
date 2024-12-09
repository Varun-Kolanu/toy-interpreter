use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use once_cell::sync::Lazy;

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
            literal: Literal::NULL,
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
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.current += 1;
                    }
                } else if self.match_next('*') {
                    self.consume_multiline_comment();
                } else {
                    self.add_non_literal_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.consume_string(),

            _ => {
                if is_digit(c) {
                    self.consume_number();
                } else if is_alpha(c) {
                    self.consume_identifier();
                } else {
                    self.had_error = true;
                    error(
                        self.line,
                        String::from(format!("Unexpected character: {}", c)),
                    )
                }
            }
        }
    }

    fn consume_multiline_comment(&mut self) {
        while !self.is_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
            let character = self.advance();
            if character == '\n' {
                self.line += 1;
            }
        }

        if self.is_at_end() {
            error(self.line, String::from("Missing trailing */ symbol."));
            self.had_error = true;
            return;
        }

        self.current += 2;
    }

    fn consume_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            let character = self.advance();
            if character == '\n' {
                self.line += 1;
            }
        }

        if self.is_at_end() {
            error(self.line, String::from("Unterminated string."));
            self.had_error = true;
            return;
        }

        self.current += 1;

        let literal = Literal::STRING(self.source[self.start + 1..self.current - 1].to_string());
        self.add_token(TokenType::STRING, literal);
    }

    fn consume_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.current += 1;

            while is_digit(self.peek()) {
                self.current += 1;
            }
        }

        let number: f64 = self.source[self.start..self.current]
            .parse()
            .expect("Failed to parse the string into double");
        self.add_token(TokenType::NUMBER, Literal::NUMBER(number));
    }

    fn consume_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.current += 1;
        }

        let identifier = &self.source[self.start..self.current];
        let mut token_type = TokenType::IDENTIFIER;

        // Keyword
        if let Some(keyword_token_type) = KEYWORDS.get(identifier) {
            token_type = keyword_token_type.clone();
        }

        self.add_non_literal_token(token_type);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        get_char(&self.source, self.current + 1)
    }

    fn advance(&mut self) -> char {
        let character = self.peek();
        self.current += 1;
        character
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        get_char(&self.source, self.current)
    }

    fn match_next(&mut self, character_to_match: char) -> bool {
        let current_char = self.peek();
        if current_char != character_to_match {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_non_literal_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::NULL);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
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

#[derive(Debug, Clone)]
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

    // Literals
    STRING,
    NUMBER,

    IDENTIFIER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::AND);
    m.insert("class", TokenType::CLASS);
    m.insert("else", TokenType::ELSE);
    m.insert("false", TokenType::FALSE);
    m.insert("for", TokenType::FOR);
    m.insert("fun", TokenType::FUN);
    m.insert("if", TokenType::IF);
    m.insert("nil", TokenType::NIL);
    m.insert("or", TokenType::OR);
    m.insert("print", TokenType::PRINT);
    m.insert("return", TokenType::RETURN);
    m.insert("super", TokenType::SUPER);
    m.insert("this", TokenType::THIS);
    m.insert("true", TokenType::TRUE);
    m.insert("var", TokenType::VAR);
    m.insert("while", TokenType::WHILE);
    m
});

enum Literal {
    STRING(String),
    NUMBER(f64),
    NULL,
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::STRING(s) => write!(f, "{}", s),
            Literal::NUMBER(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::NULL => write!(f, "null"),
        }
    }
}

#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

fn error(line: usize, message: String) {
    writeln!(io::stderr(), "[line {}] Error: {}", line, message).unwrap();
}

fn get_char(text: &str, index: usize) -> char {
    text.chars().nth(index).unwrap_or_else(|| {
        writeln!(io::stderr(), "Index out of bounds for source at {}", index).unwrap();
        '\0'
    })
}

fn is_digit(character: char) -> bool {
    character >= '0' && character <= '9'
}

fn is_alpha(character: char) -> bool {
    character == '_'
        || (character >= 'A' && character <= 'Z')
        || (character >= 'a' && character <= 'z')
}

fn is_alpha_numeric(character: char) -> bool {
    is_digit(character) || is_alpha(character)
}

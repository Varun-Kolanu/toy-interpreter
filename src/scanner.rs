use std::collections::HashMap;

use crate::token::{LiteralType, Token, TokenType};
use crate::utils::*;
use once_cell::sync::Lazy;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub had_error: bool,
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: LiteralType::NULL,
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

        let literal =
            LiteralType::STRING(self.source[self.start + 1..self.current - 1].to_string());
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
        self.add_token(TokenType::NUMBER, LiteralType::NUMBER(number));
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
        self.add_token(token_type, LiteralType::NULL);
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
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

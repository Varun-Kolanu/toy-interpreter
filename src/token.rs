use std::fmt;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
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

pub enum LiteralType {
    STRING(String),
    NUMBER(f64),
    NULL,
}

impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::STRING(s) => write!(f, "{}", s),
            LiteralType::NUMBER(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            LiteralType::NULL => write!(f, "null"),
        }
    }
}

#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

use std::io::{self, Write};

pub fn error(line: usize, message: String) {
    writeln!(io::stderr(), "[line {}] Error: {}", line, message).unwrap();
}

pub fn get_char(text: &str, index: usize) -> char {
    text.chars().nth(index).unwrap_or_else(|| {
        writeln!(io::stderr(), "Index out of bounds for source at {}", index).unwrap();
        '\0'
    })
}

pub fn is_digit(character: char) -> bool {
    character >= '0' && character <= '9'
}

pub fn is_alpha(character: char) -> bool {
    character == '_'
        || (character >= 'A' && character <= 'Z')
        || (character >= 'a' && character <= 'z')
}

pub fn is_alpha_numeric(character: char) -> bool {
    is_digit(character) || is_alpha(character)
}

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    If,
    Else,

    // Types
    TypeInt,

    // Ident and literals
    Identifier(String),
    Integer(i64),

    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Equal,

    // Arithmetic ops
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // End of file
    Eof,
}

#[derive(Debug)]
pub struct LexerError {
    message: String,
    position: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lexer error at position {}: {}",
            self.position, self.message
        )
    }
}

impl Error for LexerError {}

pub fn lex(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut position = 0;

    while let Some(&ch) = chars.peek() {
        match ch {
            ch if ch.is_whitespace() => {
                chars.next();
                position += 1;
            }
            ch if ch.is_alphabetic() => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(ch);
                        chars.next();
                        position += 1;
                    } else {
                        break;
                    }
                }

                let token = match identifier.as_str() {
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "int" => Token::TypeInt,
                    _ => Token::Identifier(identifier),
                };
                tokens.push(token);
            }
            ch if ch.is_digit(10) => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        number.push(ch);
                        chars.next();
                        position += 1;
                    } else {
                        break;
                    }
                }

                let value = number.parse::<i64>().map_err(|_| LexerError {
                    message: format!("Invalid integer: {}", number),
                    position,
                })?;
                tokens.push(Token::Integer(value));
            }
            '-' => {
                chars.next();
                position += 1;
                tokens.push(Token::Minus);
            }
            '+' => {
                chars.next();
                position += 1;
                tokens.push(Token::Plus);
            }
            '*' => {
                chars.next();
                position += 1;
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                position += 1;
                tokens.push(Token::Slash);
            }
            '=' => {
                chars.next();
                position += 1;
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    position += 1;
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal)
                }
            }
            '<' => {
                chars.next();
                position += 1;
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    position += 1;
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less)
                }
            }
            '>' => {
                chars.next();
                position += 1;
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    position += 1;
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater)
                }
            }
            '!' => {
                chars.next();
                position += 1;
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    position += 1;
                    tokens.push(Token::NotEqual);
                } else {
                    return Err(Box::new(LexerError {
                        message: "Expected '=' after '!'".to_string(),
                        position,
                    }));
                }
            }
            '(' => {
                chars.next();
                position += 1;
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                position += 1;
                tokens.push(Token::RightParen);
            }
            '{' => {
                chars.next();
                position += 1;
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                position += 1;
                tokens.push(Token::RightBrace);
            }
            ':' => {
                chars.next();
                position += 1;
                tokens.push(Token::Colon);
            }
            ';' => {
                chars.next();
                position += 1;
                tokens.push(Token::Semicolon);
            }
            _ => {
                return Err(Box::new(LexerError {
                    message: format!("Unexpected character: {}", ch),
                    position,
                }));
            }
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_function() {
        let input = "let x: int = 3; let y: int = 2; let z: int = x + y;";
        let tokens = lex(input).unwrap();
        let expected = [
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Equal,
            Token::Integer(3),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Equal,
            Token::Integer(2),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("z".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Equal,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }
}

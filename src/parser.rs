use crate::ast::{BinaryOp, ComparisonOp, Expr, Statement, Type};
use crate::lexer::Token;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn consume(&mut self, expected: Token, message: &str) -> Result<&Token, Box<dyn Error>> {
        if self.peek() == &expected {
            Ok(self.advance())
        } else {
            Err(Box::new(ParseError {
                message: message.to_string(),
            }))
        }
    }

    fn parse_type(&mut self) -> Result<Type, Box<dyn Error>> {
        match self.advance() {
            Token::TypeInt => Ok(Type::Int),
            _ => Err(Box::new(ParseError {
                message: "Expected type".to_string(),
            })),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        match self.peek() {
            Token::Let => {
                self.advance(); // consume 'let'
                let name = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    _ => {
                        return Err(Box::new(ParseError {
                            message: "Expected variable name".to_string(),
                        }))
                    }
                };
                self.consume(Token::Colon, "Expected ':' after variable name")?;
                let typ = self.parse_type()?;
                self.consume(Token::Equal, "Expected '=' after type")?;
                let value = self.parse_expression()?;
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Let { name, typ, value })
            }
            Token::Identifier(_) => {
                if let Some(Token::Equal) = self.tokens.get(self.current + 1) {
                    let name = match self.advance() {
                        Token::Identifier(name) => name.clone(),
                        _ => unreachable!(),
                    };
                    self.advance();
                    let value = self.parse_expression()?;
                    self.consume(Token::Semicolon, "Expected ';' after assignment")?;
                    Ok(Statement::Assignment {
                        target: name,
                        value,
                    })
                } else {
                    let expr = self.parse_expression()?;
                    self.consume(Token::Semicolon, "Expected ';' after expression")?;
                    Ok(Statement::Expr(expr))
                }
            }
            Token::If => self.parse_if_statement(),
            _ => {
                let expr = self.parse_expression()?;
                println!("{:?}, {}", self.peek(), self.current);
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Expr(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_binary()?;

        while matches!(
            self.peek(),
            Token::EqualEqual
                | Token::NotEqual
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
        ) {
            let op = match self.advance() {
                Token::EqualEqual => ComparisonOp::EqualEqual,
                Token::NotEqual => ComparisonOp::NotEqual,
                Token::Less => ComparisonOp::Less,
                Token::LessEqual => ComparisonOp::LessEqual,
                Token::Greater => ComparisonOp::Greater,
                Token::GreaterEqual => ComparisonOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_binary()?;
            expr = Expr::Comparison {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_binary(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_primary()?;

        while matches!(
            self.peek(),
            Token::Plus | Token::Minus | Token::Star | Token::Slash
        ) {
            let op = match self.advance() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, Box<dyn Error>> {
        let token = self.advance();
        match token {
            Token::Integer(value) => Ok(Expr::Integer(*value)),
            Token::Identifier(name) => Ok(Expr::Variable(name.clone())),
            _ => Err(Box::new(ParseError {
                message: "Expected expression".to_string(),
            })),
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        self.consume(Token::If, "Expected 'if'")?;
        self.consume(Token::LeftParen, "Expected '(' after 'if'")?;

        let condition = self.parse_expression()?;

        self.consume(Token::RightParen, "Expected ')' after if condition")?;
        self.consume(Token::LeftBrace, "Expected '{' before if body")?;

        let mut then_branch = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace, "Expected '}' after if body")?;

        let else_branch = if matches!(self.peek(), Token::Else) {
            self.advance();
            self.consume(Token::LeftBrace, "Expected '{' before else body")?;

            let mut else_statements = Vec::new();
            while !matches!(self.peek(), Token::RightBrace) {
                else_statements.push(self.parse_statement()?);
            }
            self.consume(Token::RightBrace, "Expected '}' after else body")?;

            Some(else_statements)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, Box<dyn Error>> {
    let mut parser = Parser::new(tokens);
    let mut functions = Vec::new();

    while !parser.is_at_end() {
        functions.push(parser.parse_statement()?);
    }

    Ok(functions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_parse_simple_function() {
        let input = "let x: int = 3; let y: int = 2; let z: int = x + y;";
        let tokens = lexer::lex(input).unwrap();
        let stmts = parse(tokens).unwrap();
        assert_eq!(stmts.len(), 3);
        // TODO
    }
}

use crate::ast::{BinaryOp, ComparisonOp, Expr, Function, Parameter, Statement, Type};
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

    fn parse_function(&mut self) -> Result<Function, Box<dyn Error>> {
        // Consume 'fn'
        self.consume(Token::Function, "Expected 'fn'")?;

        // Consume function name
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(Box::new(ParseError {
                    message: "Expected function name".to_string(),
                }))
            }
        };

        // Consume function params
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        let mut params = Vec::new();

        if self.peek() != &Token::RightParen {
            loop {
                let param_name = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    _ => {
                        return Err(Box::new(ParseError {
                            message: "Expected parameter name".to_string(),
                        }))
                    }
                };
                self.consume(Token::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;

                params.push(Parameter {
                    name: param_name,
                    typ: param_type,
                });

                if self.peek() != &Token::Comma {
                    break;
                }
                // Consume comma and parse next param
                self.advance();
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters")?;

        // Parse return type
        self.consume(Token::Arrow, "Expected '->' after parameters")?;
        let return_type = self.parse_type()?;

        self.consume(Token::LeftBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();
        while self.peek() != &Token::RightBrace {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after function body")?;

        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, Box<dyn Error>> {
        match self.advance() {
            Token::TypeInt => Ok(Type::Int),
            Token::TypeBool => Ok(Type::Bool),
            Token::TypeVoid => Ok(Type::Void),
            _ => Err(Box::new(ParseError {
                message: "Expected type".to_string(),
            })),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        match self.peek() {
            Token::Return => {
                self.advance(); // consume 'return'
                let expr = self.parse_expression()?;
                self.consume(Token::Semicolon, "Expected ';' after return")?;
                Ok(Statement::Return(expr))
            }
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
                self.consume(Token::Assign, "Expected '=' after type")?;
                let value = self.parse_expression()?;
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Let { name, typ, value })
            }
            Token::Identifier(_) => {
                if let Some(Token::Assign) = self.tokens.get(self.current + 1) {
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
            Token::While => self.parse_while_statement(),
            _ => {
                let expr = self.parse_expression()?;
                println!("{:?}, {}", self.peek(), self.current);
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Expr(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_logical_and()?;

        while matches!(self.peek(), Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.peek(), Token::And) {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_binary()?;

        while matches!(
            self.peek(),
            Token::Equal
                | Token::NotEqual
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
        ) {
            let op = match self.advance() {
                Token::Equal => ComparisonOp::Equal,
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
            Token::True => Ok(Expr::Boolean(true)),
            Token::False => Ok(Expr::Boolean(false)),
            Token::Identifier(name) => {
                let name = name.clone();

                if self.peek() == &Token::LeftParen {
                    // Function call
                    self.advance(); // consume '('
                    let mut args = Vec::new();

                    if self.peek() != &Token::RightParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek() != &Token::Comma {
                                break;
                            }
                            self.advance(); // consume comma
                        }
                    }

                    self.consume(Token::RightParen, "Expected ')' after arguments")?;

                    Ok(Expr::Call {
                        name: name.clone(),
                        args,
                    })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
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

    fn parse_while_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        self.consume(Token::While, "Expected 'while'")?;
        self.consume(Token::LeftParen, "Expected '(' after 'while'")?;

        let condition = self.parse_expression()?;

        self.consume(Token::RightParen, "Expected ')' after while condition")?;
        self.consume(Token::LeftBrace, "Expected '{' before while body")?;

        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace, "Expected '}' after while body")?;

        Ok(Statement::While { condition, body })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Function>, Box<dyn Error>> {
    let mut parser = Parser::new(tokens);
    let mut functions = Vec::new();

    while !parser.is_at_end() {
        functions.push(parser.parse_function()?);
    }

    Ok(functions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_parse_simple_function() {
        let input = "fn add(a: int, b: int) -> int { return a + b; }";
        let tokens = lexer::lex(input).unwrap();
        let functions = parse(tokens).unwrap();
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "add");
        assert_eq!(functions[0].params.len(), 2);
        assert_eq!(functions[0].params[0].name, "a");
        assert_eq!(functions[0].params[1].name, "b");
    }
}

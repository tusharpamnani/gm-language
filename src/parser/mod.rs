// src/parser/mod.rs

use crate::error::Rekt;
use crate::lexer::Token;
use crate::shared_types::{BinaryOp, Value, Type};
use ast::Ast;

pub mod ast;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, Rekt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(Ast::Program(statements))
    }

    fn declaration(&mut self) -> Result<Ast, Rekt> {
        match self.peek() {
            Some(Token::Launch) => self.var_declaration(false),
            Some(Token::BossFight) => self.function_declaration(),
            Some(Token::Debug) => self.var_declaration(true), // Debug is used for constant declarations
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self, is_constant: bool) -> Result<Ast, Rekt> {
        self.advance(); // consume 'launch' or 'debug'

        // Get variable name
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(Rekt::Parser("Expected variable name".to_string())),
        };

        // Check for initialization
        self.consume(&Token::Match, "Expected 'match' after variable name")?;
        let initializer = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Ast::VariableDecl {
            name,
            is_constant,
            initializer: Box::new(initializer),
        })
    }

    fn function_declaration(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'bossfight'
        
        // Get function name
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(Rekt::Parser("Expected function name".to_string())),
        };
        
        // Parse parameters
        self.consume(&Token::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => return Err(Rekt::Parser("Expected parameter name".to_string())),
                };
                
                // Optional type annotation
                let param_type = if self.check(&Token::Colon) {
                    self.advance(); // consume ':'
                    match self.advance() {
                        Some(Token::TypeInt) => Type::Number,
                        Some(Token::TypeStr) => Type::Text,
                        Some(Token::TypeBool) => Type::Boolean,
                        _ => return Err(Rekt::Parser("Expected type annotation".to_string())),
                    }
                } else {
                    Type::Number // Default to Number type
                };
                
                params.push((param_name, param_type));
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }
        self.consume(&Token::RParen, "Expected ')' after parameters")?;
        
        // Parse return type
        let return_type = if self.check(&Token::Arrow) {
            self.advance(); // consume '->'
            match self.advance() {
                Some(Token::TypeInt) => Some(Type::Number),
                Some(Token::TypeStr) => Some(Type::Text),
                Some(Token::TypeBool) => Some(Type::Boolean),
                _ => return Err(Rekt::Parser("Expected return type".to_string())),
            }
        } else {
            None
        };
        
        // Parse function body
        let body = self.block()?;
        
        Ok(Ast::FunctionDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn statement(&mut self) -> Result<Ast, Rekt> {
        match self.peek() {
            Some(Token::Ping) => self.print_statement(),
            Some(Token::Sus) => self.if_statement(),
            Some(Token::Grind) => self.while_statement(),
            Some(Token::GG) => self.return_statement(),
            Some(Token::LBrace) => Ok(Ast::Block(self.block()?)),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'ping'
        let value = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after value")?;
        Ok(Ast::PrintStmt(Box::new(value)))
    }

    fn if_statement(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'sus'
        self.consume(&Token::LParen, "Expected '(' after 'sus'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after condition")?;

        let then_branch = self.block()?;

        let else_branch = if matches!(self.peek(), Some(Token::Rekt)) {
            self.advance(); // consume 'rekt'
            Some(self.block()?)
        } else {
            None
        };

        Ok(Ast::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'grind'
        self.consume(&Token::LParen, "Expected '(' after 'grind'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after condition")?;

        let body = self.block()?;

        Ok(Ast::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn return_statement(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'gg'
        let value = if !self.check(&Token::Semicolon) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.consume(&Token::Semicolon, "Expected ';' after return")?;
        Ok(Ast::ReturnStmt(value))
    }

    fn expression_statement(&mut self) -> Result<Ast, Rekt> {
        let expr = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after expression")?;
        Ok(Ast::ExpressionStmt(Box::new(expr)))
    }

    fn block(&mut self) -> Result<Vec<Ast>, Rekt> {
        self.consume(&Token::LBrace, "Expected '{' to start block")?;
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(&Token::RBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Ast, Rekt> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Ast, Rekt> {
        let expr = self.equality()?;

        if matches!(self.peek(), Some(Token::Match)) {
            self.advance(); // consume 'match'
            let value = self.assignment()?;

            match expr {
                Ast::Variable(name) => Ok(Ast::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => Err(Rekt::Parser("Invalid assignment target".to_string())),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.match_equality_operator() {
            let right = self.comparison()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_equality_operator(&mut self) -> Option<BinaryOp> {
        if matches!(self.peek(), Some(Token::Equal)) {
            self.advance();
            Some(BinaryOp::Equal)
        } else if matches!(self.peek(), Some(Token::NotEqual)) {
            self.advance();
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }

    fn comparison(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.term()?;

        while let Some(op) = self.match_comparison_operator() {
            let right = self.term()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_comparison_operator(&mut self) -> Option<BinaryOp> {
        if matches!(self.peek(), Some(Token::GreaterThan)) {
            self.advance();
            Some(BinaryOp::Greater)
        } else if matches!(self.peek(), Some(Token::LessThan)) {
            self.advance();
            Some(BinaryOp::Less)
        } else if matches!(self.peek(), Some(Token::GreaterThanEqual)) {
            self.advance();
            Some(BinaryOp::GreaterEqual)
        } else if matches!(self.peek(), Some(Token::LessThanEqual)) {
            self.advance();
            Some(BinaryOp::LessEqual)
        } else {
            None
        }
    }

    fn term(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.factor()?;

        while let Some(op) = self.match_term_operator() {
            let right = self.factor()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_term_operator(&mut self) -> Option<BinaryOp> {
        if matches!(self.peek(), Some(Token::Plus | Token::Stake)) {
            self.advance();
            Some(BinaryOp::Add)
        } else if matches!(self.peek(), Some(Token::Minus | Token::Burn)) {
            self.advance();
            Some(BinaryOp::Subtract)
        } else {
            None
        }
    }

    fn factor(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.unary()?;

        while let Some(op) = self.match_factor_operator() {
            let right = self.unary()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_factor_operator(&mut self) -> Option<BinaryOp> {
        if matches!(self.peek(), Some(Token::Star | Token::Yield)) {
            self.advance();
            Some(BinaryOp::Multiply)
        } else if matches!(self.peek(), Some(Token::Slash | Token::Swap)) {
            self.advance();
            Some(BinaryOp::Divide)
        } else {
            None
        }
    }

    fn unary(&mut self) -> Result<Ast, Rekt> {
        if matches!(self.peek(), Some(Token::Not)) {
            self.advance();
            let expr = self.unary()?;
            Ok(Ast::Unary {
                operator: BinaryOp::Not,
                operand: Box::new(expr),
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.primary()?;

        loop {
            if matches!(self.peek(), Some(Token::LParen)) {
                self.advance();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Ast) -> Result<Ast, Rekt> {
        let mut arguments = Vec::new();

        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                arguments.push(self.expression()?);
                if !matches!(self.peek(), Some(Token::Comma)) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.consume(&Token::RParen, "Expected ')' after arguments")?;

        match callee {
            Ast::Variable(name) => Ok(Ast::Call {
                callee: name,
                arguments,
            }),
            _ => Err(Rekt::Parser("Can only call functions".to_string())),
        }
    }

    fn primary(&mut self) -> Result<Ast, Rekt> {
        if let Some(token) = self.peek() {
            let token = token.clone();
    
            match token {
                Token::Number(value) => {
                    self.advance();
                    Ok(Ast::Literal(Value::Token(value.to_string())))
                },
                Token::Text(text) => {
                    self.advance();
                    Ok(Ast::Literal(Value::Text(text.clone())))
                },
                Token::True => {
                    self.advance();
                    Ok(Ast::Literal(Value::Signal(true)))
                },
                Token::False => {
                    self.advance();
                    Ok(Ast::Literal(Value::Signal(false)))
                },
                Token::Identifier(name) => {
                    self.advance();
                    Ok(Ast::Variable(name.clone()))
                },
                Token::LParen => {
                    self.advance();
                    let expr = self.expression()?;
                    self.consume(&Token::RParen, "Expected ')' after expression")?;
                    Ok(Ast::Grouping(Box::new(expr)))
                },
                _ => Err(Rekt::Parser(format!("Unexpected token: {}", token))),
            }
        } else {
            Err(Rekt::Parser("Unexpected end of input".to_string()))
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn check(&self, token: &Token) -> bool {
        matches!(self.peek(), Some(t) if t == token)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), None)
    }

    fn consume(&mut self, expected: &Token, message: &str) -> Result<(), Rekt> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(Rekt::Parser(message.to_string()))
        }
    }
    #[allow(dead_code)]
    fn match_token(&mut self, token: Token) -> bool {
        if let Some(current_token) = self.tokens.get(self.current) {
            if *current_token == token {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn binary(&mut self) -> Result<Ast, Rekt> {
        let mut expr = self.unary()?;
    
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus | Token::Stake => BinaryOp::Add,
                Token::Minus | Token::Burn => BinaryOp::Subtract,
                Token::Star | Token::Yield => BinaryOp::Multiply,
                Token::Slash | Token::Swap => BinaryOp::Divide,
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                Token::GreaterThan => BinaryOp::Greater,
                Token::LessThan => BinaryOp::Less,
                Token::GreaterThanEqual => BinaryOp::GreaterEqual,
                Token::LessThanEqual => BinaryOp::LessEqual,
                Token::And => BinaryOp::And,
                Token::Or => BinaryOp::Or,
                _ => break,
            };
    
            self.advance();
            let right = self.unary()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
    
        Ok(expr)
    }
}


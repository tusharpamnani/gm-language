// src/parser/mod.rs

use crate::error::Rekt;
use crate::lexer::Token;
use crate::shared_types::{BinaryOp, Value};
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
            Some(Token::Launch) => self.var_declaration(),
            Some(Token::Loop) => self.declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Ast, Rekt> {
        self.advance(); // consume 'launch'

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(Rekt::Parser("Expected variable name".to_string())),
        };

        self.consume(&Token::Match, "Expected 'match' after variable name")?; // assuming 'match' is the keyword for initialization
        let initializer = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Ast::VariableDecl {
            name,
            is_constant: false,
            initializer: Box::new(initializer),
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
        let expr = self.or()?;

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

    fn or(&mut self) -> Result<Ast, Rekt> {
        let mut expr1 = self.and()?; // Parse the left side of the `or` expression
        
        while self.match_token(Token::Or) { // Check for `||` operator
            let expr2 = self.and()?; // Parse the right side of the `or` expression
            expr1 = Ast::Binary {
                left: Box::new(expr1),
                operator: BinaryOp::Or, // Use `Or` operator for `||`
                right: Box::new(expr2),
            };
        }
        
        Ok(expr1)
    }

    fn and(&mut self) -> Result<Ast, Rekt> {
        let mut expr1 = self.primary()?; // Parse a primary expression first (e.g., a value or variable)

        while self.match_token(Token::And) { // Check for `&&` operator
            let expr2 = self.primary()?;
            expr1 = Ast::Binary {
                left: Box::new(expr1),
                operator: BinaryOp::And, // Use `And` operator for `&&`
                right: Box::new(expr2),
            };
        }

        Ok(expr1)
    }

    fn primary(&mut self) -> Result<Ast, Rekt> {
        if let Some(token) = self.peek() {
            // First handle the immutable borrow by storing `peek()` in a local variable
            let token = token.clone();
    
            match token {
                Token::Number(value) => {
                    self.advance();
                    Ok(Ast::Literal(Value::Number(value.clone() as f64)))
                },
                Token::Text(text) => {
                    self.advance();
                    Ok(Ast::Literal(Value::Text(text.clone())))
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
                _ => Err(Rekt::Parser("Unexpected token".to_string())),
            }
            
        } else {
            Err(Rekt::Parser("Unexpected token".to_string())) // Handle the case where peek() is None
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

    fn match_token(&mut self, token: Token) -> bool {
        if let Some(current_token) = self.tokens.get(self.current) {
            if *current_token == token {
                self.current += 1;
                return true;
            }
        }
        false
    }
}

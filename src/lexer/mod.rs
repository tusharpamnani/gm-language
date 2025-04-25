use logos::Logos;
use crate::error::Rekt;
pub use token::Token;

mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Rekt> {
        let mut tokens = Vec::new();
        let mut lexer = Token::lexer(self.source);

        while let Some(token) = lexer.next() {
            let span = lexer.span();
            // println!(
            //     "🔍 Token: {:?} | slice: '{}' | span: {:?}",
            //     token,
            //     &self.source[span.start..span.end],
            //     span
            // );
        
            match token {
                Token::Rekt => {
                    return Err(Rekt::Lexer(format!(
                        "yo, invalid token '{}' at line {}, column {} 🤕",
                        &self.source[span.start..span.end], self.line, self.column
                    )));
                }
                token => tokens.push(token),
            }
                
        }

        Ok(tokens)
    }
}

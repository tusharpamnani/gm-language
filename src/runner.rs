use std::fs;
use std::path::Path;
use colored::*;

// Import the new Rekt instead of LoveError
use crate::error::Rekt;  
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::fun::*;  // Assuming the fun module is still relevant

pub struct Runner {
    interpreter: Interpreter,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Rekt> {
        let path = path.as_ref();
        
        // Check file extension
        if let Some(extension) = path.extension() {
            if extension != "gm" {  // Changed to .gm extension for "Gm"
                return Err(Rekt::Runtime(
                    "Only .gm files can contain our gm story! 💚".to_string()
                ));
            }
        } else {
            return Err(Rekt::Runtime(
                "File must have a .gm extension! 💚".to_string()
            ));
        }

        // Read file content
        let content = fs::read_to_string(path)
            .map_err(|e| Rekt::Runtime(format!("Failed to read gm letter: {}", e)))?;

        println!("{}", create_gm_border(
            &format!("💌 Reading gm story from: {}", path.display())
        ).bright_cyan());

        // Print the content being executed
        println!("{}", "💚 Gm story output:".bright_yellow());
        println!();

        // Execute the code
        let mut lexer = Lexer::new(&content);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", create_gm_border(
                    &format!("💔 Lexer error:\n{}", format_error(&e))
                ).bright_red());
                return Err(e);
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                println!("{}", create_gm_border(
                    &format!("💔 Parser error:\n{}", format_error(&e))
                ).bright_red());
                return Err(e);
            }
        };

        match self.interpreter.interpret(ast) {
            Ok(_) => {
                println!("{}", create_gm_border(
                    &format!("{} Gm story executed successfully!", get_random_emoji())
                ).bright_green());
                Ok(())
            },
            Err(e) => {
                println!("{}", create_gm_border(
                    &format!("💔 Runtime error:\n{}", format_error(&e))
                ).bright_red());
                Err(e)
            }
        }
    }
}

// Updated to Rekt handling
fn format_error(error: &Rekt) -> String {
    match error {
        Rekt::Lexer(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Parser(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Runtime(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Type(msg) => format!("{}\n{}", get_random_error_message(), msg),
    }
}

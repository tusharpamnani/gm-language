use rustyline::Editor;
use colored::*;
use std::io::{self};
use std::env;

mod shared_types;
mod lexer;
mod parser;
mod interpreter;
mod error;
mod fun;
mod runner;

use crate::runner::Runner;
use crate::shared_types::Value;
use crate::error::Rekt;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::fun::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_repl(),
        
        2 => run_file(&args[1]),

        _ => {
            println!("{}", create_gm_border(
                "Usage: gm-language [script.gm]"
            ).bright_red());
            Ok(())
        }
    }
}

fn run_repl() -> io::Result<()> {
    print_welcome_message();

    let mut rl = Editor::<()>::new();
    let mut interpreter = interpreter::Interpreter::new();
    let mut current_line = String::new();
    let mut brace_count = 0;

    loop {
        let prompt = if brace_count > 0 { 
            format!("{}  ", get_random_emoji()) 
        } else { 
            format!("{}> ", get_random_emoji())
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed_line = line.trim();

                if handle_special_commands(trimmed_line) {
                    continue;
                }

                if trimmed_line.eq_ignore_ascii_case("gm_break;") {
                    println!("{}", "Goodbye! The GM journey pauses...".bright_red()); // Fixed here
                    break;
                }

                brace_count += count_braces(trimmed_line);
                
                current_line.push_str(&line);
                current_line.push('\n');

                if brace_count == 0 && !trimmed_line.is_empty() && 
                   !trimmed_line.ends_with(';') && !trimmed_line.ends_with('{') && 
                   !trimmed_line.ends_with('}') && !current_line.contains("blockchain") {
                    println!("{}", "💥 Missing semicolon at end of statement".bright_red()); // Fixed here
                    current_line.clear();
                    continue;
                }

                if brace_count == 0 && (trimmed_line.ends_with(';') || trimmed_line.ends_with('}')) {
                    match validate_syntax(&current_line) {
                        Ok(_) => {
                            rl.add_history_entry(current_line.as_str());

                            match execute_line(&current_line, &mut interpreter) {
                                Ok(value) => {
                                    match value {
                                        Value::Null => (),
                                        _ => {
                                            println!("{} {}", get_random_emoji(), get_random_success_message().bright_green());
                                            println!("{} {:?}", get_random_emoji(), value)
                                        },
                                    }
                                },
                                Err(e) => println!("{}", format!("💥 Error: {}", format_error(&e)).bright_red()), // Fixed here
                            }
                        }
                        Err(e) => println!("{}", format!("💥 Error: {}", format_error(&e)).bright_red()), // Fixed here
                    }
                    
                    current_line.clear();
                } else if brace_count < 0 {
                    println!("{}", "💥 Unmatched closing brace".bright_red()); // Fixed here
                    current_line.clear();
                    brace_count = 0;
                }
            }
            Err(err) => {
                println!("{}", format!("💥 Error: {}", format!("{}\n{}", get_random_error_message(), err.to_string())).bright_red()); // Fixed here
                break;
            }
        }
    }

    Ok(())
}



fn run_file(path: &str) -> io::Result<()> {
    let mut runner = Runner::new();

    // Simplified message format for reading gm script
    println!("{}", format!("Reading GM script from: {}", path).bright_blue());
    
    if let Err(e) = runner.run_file(path) {
        // Simplified error format
        println!("💥 The GM chain broke down: {}", e);
    }
    
    // Simplified message format after the operation
    println!("{}", "GM script executed successfully!".bright_green());

    Ok(())
}


fn print_welcome_message() {
    println!("{}", r#"
╭━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╮
│     Welcome to GM Language! 💎     │
│  Where Blockchain & GM Unite! 🚀    │
╰━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╯
"#.bright_magenta());
    
    println!("{}", "💡 Dear coder, let the GM shine in your code...".bright_cyan());
    println!("{}", "\n🚀 Quick Reference:".bright_yellow());
    println!("   gem x token 10;           // Variables represent tokens");
    println!("   blockCHAIN GM match 100;  // Constants in the blockchain");
    println!("   whisper \"GM!\";          // Share a message to the chain");
    println!("   x stake y                // Addition = Stake multiplication");
    println!("   x mine y                 // Multiplication = Blockchain magic");
    println!("\n💥 Type 'gm_break;' to end the GM connection...");
    println!("💡 Type 'gm help' for more info\n");
}

fn execute_line(line: &str, interpreter: &mut interpreter::Interpreter) -> Result<Value, Rekt> {
    if line.trim().is_empty() {
        return Ok(Value::Null);
    }

    let mut lexer = lexer::Lexer::new(line);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let result = interpreter.interpret(ast)?;
    
    // Add fun messages based on the code being executed
    if line.contains("blockchain") {
        println!("{}", create_gm_border("New blockchain function deployed! 🚀").bright_cyan());
    } else if line.contains("token") {
        println!("{} {}", get_random_emoji(), "A new token found on the chain!".bright_green());
    }
    
    Ok(result)
}

fn validate_syntax(input: &str) -> Result<(), Rekt> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()?;
    Ok(())
}

fn count_braces(line: &str) -> i32 {
    let mut count = 0;
    for c in line.chars() {
        match c {
            '{' => count += 1,
            '}' => count -= 1,
            _ => (),
        }
    }
    count
}

fn format_error(error: &Rekt) -> String {
    match error {
        Rekt::Lexer(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Parser(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Runtime(msg) => format!("{}\n{}", get_random_error_message(), msg),
        Rekt::Type(msg) => format!("{}\n{}", get_random_error_message(), msg),
    }
    .bright_red() // Apply bright_red() to the formatted string here
    .to_string()  // Convert the colorized string back to a String
}



fn create_gm_border(text: &str) -> String {
    let border = "╭━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╮";
    let bottom = "╰━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╯";
    // let padding = "│                                    │";

    format!(
        "{}\n {}\n {}",
        border,
        // padding,
        text,
        // padding,
        bottom
    )
}

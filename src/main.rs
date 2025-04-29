use rustyline::Editor;
use colored::*;
use std::io::{self};
use std::env;
#[allow(unused_imports)]
use std::fs;

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
#[allow(unused_imports)]
use crate::fun::*;

fn main() -> io::Result<()> {
    // Clear the terminal screen as the first action
    clear_screen();
    
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_repl(),
        
        2 => run_file(&args[1]),

        _ => {
            println!("{}", create_crypto_border(
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
                    println!("{}", "Goodbye! The GM journey pauses...".bright_red());
                    break;
                }

                brace_count += count_braces(trimmed_line);
                
                current_line.push_str(&line);
                current_line.push('\n');

                if brace_count == 0 && !trimmed_line.is_empty() && 
                   !trimmed_line.ends_with(';') && !trimmed_line.ends_with('{') && 
                   !trimmed_line.ends_with('}') && !current_line.contains("mine") {
                    println!("{}", "💥 Missing semicolon at end of statement".bright_red());
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
                                Err(e) => println!("{}", format!("💥 Error: {}", format_error(&e)).bright_red()),
                            }
                        }
                        Err(e) => println!("{}", format!("💥 Error: {}", format_error(&e)).bright_red()),
                    }
                    
                    current_line.clear();
                } else if brace_count < 0 {
                    println!("{}", "💥 Unmatched closing brace".bright_red());
                    current_line.clear();
                    brace_count = 0;
                }
            }
            Err(err) => {
                println!("{}", format!("💥 Error: {}", format!("{}\n{}", get_random_error_message(), err.to_string())).bright_red());
                break;
            }
        }
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let mut runner = Runner::new();

    println!("{}", format!("Reading GM script from: {}", path).bright_blue());
    
    if let Err(e) = runner.run_file(path) {
        println!("💥 The GM chain broke down: {}", e);
    } else {
        println!("{}", "GM script executed successfully!".bright_green());
    }
    
    Ok(())
}

// Cross-platform function to clear the terminal screen
fn clear_screen() {
    // For Windows
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    } 
    // For Unix-like systems (Linux, macOS)
    else {
        let _ = std::process::Command::new("clear")
            .status();
    }
    
    // Fallback using ANSI escape codes (works in most modern terminals)
    print!("\x1B[2J\x1B[1;1H");
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
    println!("   token x = 10;            // Variables represent tokens");
    println!("   block DIAMOND = 100;     // Constants in the blockchain");
    println!("   ping \"GM!\";            // Share a message to the chain");
    println!("   x stake y;               // Addition operation");
    println!("   x yield y;               // Multiplication operation");
    println!("   x burn y;                // Subtraction operation");
    println!("   x swap y;                // Division operation");
    println!("\n   mine greet(name) {{         // Define a function"); 
    println!("     ping \"GM \" + name;");
    println!("     return name;");
    println!("   }}");  // Double curly braces to escape
    
    println!("\n💥 Type 'gm_break;' to end the GM connection...");
    println!("💡 Type 'crypto help' for more info\n");
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
    if line.contains("mine") {
        println!("{}", create_crypto_border("New blockchain function deployed! 🚀").bright_cyan());
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
        if c == '{' {
            count += 1;
        } else if c == '}' {
            count -= 1;
        }
    }
    count
}

fn handle_special_commands(command: &str) -> bool {
    match command {
        "crypto help" => {
            print_help_message();
            true
        },
        "clear" | "cls" => {
            // Clear screen using our cross-platform function
            clear_screen();
            true
        },
        "examples" => {
            print_examples();
            true
        },
        _ => false
    }
}

fn print_help_message() {
    println!("{}", create_crypto_border("GM Language Help").bright_yellow());
    println!("🚀 GM Language is inspired by blockchain concepts.");
    println!("💡 Core Features:");
    println!("   - Variables are called 'tokens'");
    println!("   - Constants are called 'blocks'");
    println!("   - Functions are declared with 'mine'");
    println!("   - Print with 'ping'");
    println!("   - Standard math operations: stake (+), burn (-), yield (*), swap (/)");
    println!("   - Control flow: if/else, while loops");
    println!("\n💼 Special Commands:");
    println!("   - crypto help: Show this help");
    println!("   - examples: Show code examples");
    println!("   - clear/cls: Clear the screen");
    println!("   - gm_break;: Exit the REPL");
    println!("{}", create_crypto_border("Happy mining!").bright_yellow());
}

fn print_examples() {
    println!("{}", create_crypto_border("GM Language Examples").bright_cyan());
    
    println!("// Basic variable declaration");
    println!("token balance = 100;");
    println!("token name = \"Satoshi\";");
    println!("");
    
    println!("// Constants");
    println!("block MAX_SUPPLY = 21000000;");
    println!("");
    
    println!("// Simple operations");
    println!("token a = 5;");
    println!("token b = 10;");
    println!("token sum = a stake b;  // Addition: 15");
    println!("token diff = b burn a;  // Subtraction: 5");
    println!("token product = a yield b;  // Multiplication: 50");
    println!("token quotient = b swap a;  // Division: 2");
    println!("");
    
    println!("// Function definition");
    println!("mine calculate_gas(amount, rate) {{");
    println!("    return amount yield rate;");
    println!("}}");
    println!("");
    
    println!("// Conditional statements");
    println!("token eth = 10;");
    println!("if (eth > 5) {{");
    println!("    ping \"High ETH balance!\";");
    println!("}} else {{");
    println!("    ping \"Low ETH balance.\";");
    println!("}}");
    println!("");
    
    println!("// Loops");
    println!("token i = 0;");
    println!("while (i < 5) {{");
    println!("    ping \"Mining block \" + i;");
    println!("    i = i stake 1;");
    println!("}};");
    
    println!("{}", create_crypto_border("Try them out!").bright_cyan());
}

fn create_crypto_border(text: &str) -> String {
    let width = text.len() + 6;
    let border = "=".repeat(width);
    format!("\n{}\n🔗 {} 🔗\n{}\n", border, text, border)
}

fn get_random_emoji() -> &'static str {
    let emojis = [
        "💎", "🚀", "⛓️", "🔐", "💰", "🌐", "📈", "🔗", "🧠", "✨", 
        "🛡️", "💹", "🔥", "⚡", "🌟", "🏆", "🤑", "💯", "🔱", "📊"
    ];
    
    let idx = rand::random::<usize>() % emojis.len();
    emojis[idx]
}

fn get_random_success_message() -> &'static str {
    let messages = [
        "Block mined successfully!",
        "Token verified on chain!",
        "Smart contract deployed!",
        "Transaction confirmed!",
        "Consensus achieved!",
        "Hash computed correctly!",
        "Wallet synced perfectly!",
        "Node updated successfully!",
        "Mining rewards collected!",
        "Blockchain integrity verified!"
    ];
    
    let idx = rand::random::<usize>() % messages.len();
    messages[idx]
}

fn get_random_error_message() -> &'static str {
    let messages = [
        "Transaction failed!",
        "Gas limit exceeded!",
        "Consensus failure!",
        "Chain fork detected!",
        "Mining difficulty too high!",
        "Smart contract error!",
        "Token validation failed!",
        "Block rejected by nodes!",
        "Wallet connection lost!",
        "Hash mismatch detected!"
    ];
    
    let idx = rand::random::<usize>() % messages.len();
    messages[idx]
}

fn format_error(err: &Rekt) -> String {
    match err {
        Rekt::Lexer(msg) => format!("Lexer Error: {}", msg),
        Rekt::Parser(msg) => format!("Parser Error: {}", msg),
        Rekt::Runtime(msg) => format!("Runtime Error: {}", msg),
        Rekt::Type(msg) => format!("Type Error: {}", msg),
    }
}
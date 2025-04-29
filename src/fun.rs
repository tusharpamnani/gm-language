#[allow(dead_code)]
use colored::*;
use rand::Rng;

// Crypto-themed emojis and messages
pub const CRYPTO_EMOJIS: &[&str] = &["💎", "🪙", "🔥", "🚀", "⚡", "💡", "🪐", "💰", "📈", "🌕", "💸", "🌍"];
#[allow(dead_code)]
pub const SUCCESS_MESSAGES: &[&str] = &[
    "To the moon! 🚀",
    "Your code is on fire! 🔥",
    "What a bullish expression! 📈",
    "You're coding like a true Degen! 💎",
    "Your heart is mining blocks of love! 💰",
    "Such decentralized programming! 🌍",
    "Code to the moon, let's HODL! 🌕",
    "Your code is worth its weight in ETH! 🤑",
];

pub const ERROR_MESSAGES: &[&str] = &[
    "Oh no, gas fees too high! 💸",
    "Blockchain error... Block too full! ⚡",
    "Not all crypto journeys are smooth... ⛔",
    "Even the best chains have forks... 🔗",
    "Time to fix the smart contract... 🛠️",
    "GM is patient, GM is decentralized, but this code needs gas! ⛽",
];

pub fn get_random_emoji() -> String {
    let mut rng = rand::thread_rng();
    CRYPTO_EMOJIS[rng.gen_range(0..CRYPTO_EMOJIS.len())].to_string()
}

#[allow(dead_code)]
pub fn get_random_success_message() -> String {
    let mut rng = rand::thread_rng();
    SUCCESS_MESSAGES[rng.gen_range(0..SUCCESS_MESSAGES.len())].to_string()
}

pub fn get_random_error_message() -> String {
    let mut rng = rand::thread_rng();
    ERROR_MESSAGES[rng.gen_range(0..ERROR_MESSAGES.len())].to_string()
}

pub fn create_crypto_border(message: &str) -> String {
    let width = message.len() + 4;
    let border: String = "🚀".repeat(width);
    format!("{}
🚀 {} 🚀
{}", border, message, border)
}

pub fn create_gm_border(message: &str) -> String {
    let width = message.len() + 4;
    let border: String = "💚".repeat(width);
    format!("{}
💚 {} 💚
{}", border, message, border)
}

#[allow(dead_code)]
pub fn print_crypto_help() {
    println!("{}", create_crypto_border("🌍 Crypto Language Quick Guide 🌍").bright_cyan());
    println!("
🌟 Special Commands:
   crypto help    - Show this guide
   crypto story   - Tell a random crypto journey
   crypto quote   - Share a crypto quote

🎨 Basic Syntax:
   token       -> declare variables (ERC20/BEP20)
   block       -> declare constants (Immutable)
   mine        -> define functions (Smart Contracts)
   broadcast   -> print values (Events)
   
💎 Operators:
   stake      -> addition (+)
   yield      -> multiply (*)
   burn       -> subtract (-)
   swap       -> divide (/)
");
}

#[allow(dead_code)]
pub fn print_random_crypto_quote() {
    let quotes = [
        "In code as in crypto, simplicity is decentralized.",
        "Every smart contract is a trustless relationship.",
        "Bug-free code is worth its weight in ETH.",
        "Variables may be undefined, but our crypto journey isn’t.",
        "The best code is written with decentralized energy."
    ];
    let mut rng = rand::thread_rng();
    let quote = quotes[rng.gen_range(0..quotes.len())];
    println!("{}", create_crypto_border(quote).bright_magenta());
}

#[allow(dead_code)]
pub fn handle_special_commands(line: &str) -> bool {
    match line.trim() {
        "crypto help" => {
            print_crypto_help();
            true
        },
        "crypto quote" => {
            print_random_crypto_quote();
            true
        },
        _ => false
    }
}

# GM Language 💎

GM Language is a fun, crypto-themed programming language that incorporates crypto slangs into coding! Express your algorithms with Web3-inspired keywords and operators.

## 🌟 Features

- Crypto-themed keywords and operators
- Variables (tokens) and constants (blocks)
- Smart contracts (functions) with parameters and return values
- Control flow statements (if-else, while)
- Basic arithmetic operations
- String operations
- Boolean logic (signals)
- Blockchain-inspired syntax

## 💖 Quick Start
### Installation

1. Make sure you have **Rust** installed on your system. If not, [download Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository:

```bash
git clone https://github.com/tusharpamnani/gm-language.git
cd gm-language
```

3. Build the project:

```bash
cargo build --release
```

### Running GM Language Programs

You can run GM Language in two modes:

1. **Interactive REPL** (Read-Eval-Print Loop):

```bash
cargo run
```

2. **Run a .gm script**:

```bash
cargo run -- path/to/your/script.gm
```

## 📚 Language Basics

### Variables and Constants
```
token balance = 100;        // Variable declaration
block MAX_SUPPLY = 21000000;  // Constant declaration
```

### Operators
```
// Arithmetic
token a = 5;
token b = 10;
token sum = a stake b;      // Addition (5 + 10 = 15)
token diff = b burn a;      // Subtraction (10 - 5 = 5)
token product = a yield b;  // Multiplication (5 * 10 = 50)
token quotient = b swap a;  // Division (10 / 5 = 2)

// Comparison
token is_equal = (a == b);  // Equality check
token is_greater = (b > a); // Greater than
```

### Functions (Smart Contracts)
```
mine calculate_gas(amount, rate) {
    return amount yield rate;
}

// Function call
token gas_fee = calculate_gas(10, 5);
```

### Control Flow
```
// Conditional statements
sus (balance > 100) {
    broadcast "High balance!";
} rekt {
    broadcast "Low balance.";
}

// Loops
token i = 0;
grind (i < 5) {
    broadcast "Mining block " + i;
    i = i stake 1;
}
```

## Current Status

The GM compiler supports basic crypto-themed programming with variables, functions, control flow, and arithmetic operations. It features a REPL environment and can run script files.

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Add new crypto-themed operators
- Improve documentation
- Create example programs

## 💌 Contact

For questions, suggestions, or messages about the language, please open an issue on GitHub.

Remember: Code with GM spirit! 💎
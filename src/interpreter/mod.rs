use std::collections::HashMap;
use crate::shared_types::{ BinaryOp, Type, Value };
use crate::parser::ast::Ast;
use crate::error::Rekt;

// number => token
// boolean => signal
// function => smart contract
// printstmt => emit event
// define => mint
// assign => transfer
// binaryOp::Add => airdrop
// Error:Runtime => Rekt

// environment
pub struct Wallet {
    tokens: HashMap<String, Value>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            tokens: HashMap::new(),
        }
    }

    pub fn mint(&mut self, address: String, token: Value) {
        self.tokens.insert(address, token);
    }

    pub fn fetch(&self, address: &str) -> Option<&Value> {
        self.tokens.get(address)
    }

    pub fn transfer(&mut self, address: &str, token: Value) -> Result<(), Rekt> {
        if self.tokens.contains_key(address) {
            self.tokens.insert(address.to_string(), token);
            Ok(())
        } else {
            Err(Rekt::Runtime(format!("Wallet address '{}' not found.", address)))
        }
    }
}

// interpreter
pub struct Interpreter {
    wallet: Wallet,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            wallet: Wallet::new(),
        }
    }

     // The interpret method
     pub fn interpret(&mut self, ast: Ast) -> Result<Value, Rekt> {
        self.deploy(ast)?;  // Assuming deploy returns Result<Value, Rekt>
        Ok(Value::Null)  // Or other appropriate Value
    }
    
    

    fn verify_type(&self, token: &Value, expected: Type) -> Result<(), Rekt> {
        let actual = token.get_type();
        if actual != expected {
            Err(Rekt::Type(format!("Expected {}, but found {}", expected, token)))
        } else {
            Ok(())
        }
    }

    fn validate_tx(
        &self,
        left: &Value,
        right: &Value,
        _op: &BinaryOp,
        expected_type: Type
    ) -> Result<(), Rekt> {
        self.verify_type(left, expected_type.clone())?;
        self.verify_type(right, expected_type.clone())?;
        Ok(())
    }

    pub fn deploy(&mut self, ast: Ast) -> Result<Value, Rekt> {
        match ast {
            Ast::Program(contracts) => {
                let mut result = Value::Null;
                for c in contracts {
                    result = self.deploy(c)?;
                }
                Ok(result)
            }

            Ast::FunctionDecl { name, params, body, .. } => {
                let arg_names = params
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect();
                let contract = Value::SmartContract {
                    name: name.clone(),
                    params: arg_names,
                    body,
                };
                self.wallet.mint(name, contract.clone());
                Ok(contract)
            }

            Ast::Call { callee, arguments } => {
                let contract = self.wallet
                    .fetch(&callee)
                    .ok_or_else(|| Rekt::Lexer(format!("Unknown smart contract '{}'", callee)))?
                    .clone();

                match contract {
                    Value::SmartContract { params, body, .. } => {
                        let mut new_wallet = Wallet::new();

                        if params.len() != arguments.len() {
                            return Err(
                                Rekt::Lexer(
                                    format!(
                                        "Expected {} args, got {}.",
                                        params.len(),
                                        arguments.len()
                                    )
                                )
                            );
                        }

                        for (param, arg) in params.iter().zip(arguments) {
                            let val = self.deploy(arg)?;
                            new_wallet.mint(param.clone(), val);
                        }

                        let old_wallet = std::mem::replace(&mut self.wallet, new_wallet);
                        let mut result = Value::Null;
                        for stmt in body {
                            result = self.deploy(stmt)?;
                        }
                        self.wallet = old_wallet;
                        Ok(result)
                    }
                    _ => Err(Rekt::Lexer(format!("'{}' is not a contract", callee))),
                }
            }

            Ast::ReturnStmt(val) => {
                match val {
                    Some(expr) => self.deploy(*expr),
                    None => Ok(Value::Null),
                }
            }

            Ast::VariableDecl { name, initializer, .. } => {
                let token = self.deploy(*initializer)?;
                self.wallet.mint(name, token.clone());
                Ok(token)
            }

            Ast::If { condition, then_branch, else_branch } => {
                let signal = self.deploy(*condition)?;
                match signal {
                    Value::Signal(true) => {
                        let mut last = Value::Null;
                        for stmt in then_branch {
                            last = self.deploy(stmt)?;
                        }
                        Ok(last)
                    }
                    Value::Signal(false) => {
                        if let Some(else_stmts) = else_branch {
                            let mut last = Value::Null;
                            for stmt in else_stmts {
                                last = self.deploy(stmt)?;
                            }
                            Ok(last)
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    _ =>
                        Err(
                            Rekt::Lexer("Condition must be bullish or bearish (yes/no)".to_string())
                        ),
                }
            }

            Ast::Binary { left, operator, right } => {
                let l = self.deploy(*left)?;
                let r = self.deploy(*right)?;
            
                match operator {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                        self.validate_tx(&l, &r, &operator, Type::Token)?;
            
                        // Try parsing the values to f64 before applying operations
                        if let (Value::Token(a), Value::Token(b)) = (l, r) {
                            // Parse both a and b as f64
                            if let Ok(a_val) = a.parse::<f64>() {
                                if let Ok(b_val) = b.parse::<f64>() {
                                    match operator {
                                        BinaryOp::Add => Ok(Value::Token((a_val + b_val).to_string())),
                                        BinaryOp::Subtract => Ok(Value::Token((a_val - b_val).to_string())),
                                        BinaryOp::Multiply => Ok(Value::Token((a_val * b_val).to_string())),
                                        BinaryOp::Divide => {
                                            if b_val == 0.0 {
                                                Err(Rekt::Lexer("You just got rekt by zero!".to_string()))
                                            } else {
                                                Ok(Value::Token((a_val / b_val).to_string()))
                                            }
                                        }
                                        _ => Err(Rekt::Lexer("Invalid trade".to_string())),
                                    }
                                } else {
                                    Err(Rekt::Lexer("Invalid number: b".to_string())) // Handle invalid `b` parsing
                                }
                            } else {
                                Err(Rekt::Lexer("Invalid number: a".to_string())) // Handle invalid `a` parsing
                            }
                        } else {
                            Err(Rekt::Lexer("Invalid trade".to_string())) // Handle invalid types in left and right values
                        }
                    }
                    BinaryOp::Greater
                    | BinaryOp::Less
                    | BinaryOp::GreaterEqual
                    | BinaryOp::LessEqual => {
                        self.validate_tx(&l, &r, &operator, Type::Token)?;
            
                        match (l, operator, r) {
                            (Value::Token(a), BinaryOp::Greater, Value::Token(b)) =>
                                Ok(Value::Signal(a > b)),
                            (Value::Token(a), BinaryOp::Less, Value::Token(b)) =>
                                Ok(Value::Signal(a < b)),
                            (Value::Token(a), BinaryOp::GreaterEqual, Value::Token(b)) =>
                                Ok(Value::Signal(a >= b)),
                            (Value::Token(a), BinaryOp::LessEqual, Value::Token(b)) =>
                                Ok(Value::Signal(a <= b)),
                            _ => Err(Rekt::Lexer("Invalid comparison".to_string())),
                        }
                    }
                    BinaryOp::Equal => {
                        match (l, r) {
                            (Value::Token(a), Value::Token(b)) => Ok(Value::Signal(a == b)),
                            (Value::Signal(a), Value::Signal(b)) => Ok(Value::Signal(a == b)),
                            _ => Err(Rekt::Lexer("Cannot compare different types".to_string())),
                        }
                    }
                    BinaryOp::NotEqual => {
                        match (l, r) {
                            (Value::Token(a), Value::Token(b)) => Ok(Value::Signal(a != b)),
                            (Value::Signal(a), Value::Signal(b)) => Ok(Value::Signal(a != b)),
                            _ => Err(Rekt::Lexer("Cannot compare different types".to_string())),
                        }
                    }
                    _ => Err(Rekt::Lexer("Op not supported yet.".to_string())),
                }
            }
            

            Ast::PrintStmt(expr) => {
                let val = self.deploy(*expr)?;
                println!("📢 Event: {:?}", val);
                Ok(Value::Null)
            }

            Ast::Literal(val) => Ok(val),
            Ast::Variable(name) => {
                self.wallet
                    .fetch(&name)
                    .cloned()
                    .ok_or_else(|| Rekt::Lexer(format!("Token '{}' not found.", name)))
            }
            Ast::Assign { name, value } => {
                let eval = self.deploy(*value)?;
                self.wallet.transfer(&name, eval.clone())?;
                Ok(eval)
            }
            Ast::ExpressionStmt(expr) => self.deploy(*expr),
            Ast::Grouping(expr) => self.deploy(*expr),
            Ast::Block(stmts) => {
                let mut result = Value::Null;
                let new_wallet = Wallet::new();
                let old_wallet = std::mem::replace(&mut self.wallet, new_wallet);

                for stmt in stmts {
                    result = self.deploy(stmt)?;
                }

                self.wallet = old_wallet;
                Ok(result)
            }

            _ => Err(Rekt::Lexer("Unimplemented in GmScript.".to_string())),
        }
    }
}

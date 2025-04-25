use crate::shared_types::{Value, BinaryOp, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Vec<Ast>),

    // Variables (declaring meme-worthy variables)
    VariableDecl {
        name: String,
        initializer: Box<Ast>,
        is_constant: bool, // Should this meme be locked? 👀
    },

    // Functions (Epic Functions like boss fights)
    FunctionDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Vec<Ast>,
    },

    // Function Calls (let's ping that function)
    Call {
        callee: String, // The meme machine
        arguments: Vec<Ast>,
    },

    // Conditional checks (Are we sus?)
    If {
        condition: Box<Ast>, // Condition to check
        then_branch: Vec<Ast>, // Action if it’s cool
        else_branch: Option<Vec<Ast>>, // Action if it’s sus
    },

    // While loop (Grind till it’s done)
    While {
        condition: Box<Ast>, // Loop condition
        body: Vec<Ast>, // Keep grinding
    },

    // Code blocks (meme sequences)
    Block(Vec<Ast>),

    // Expressions (because devs love side effects)
    ExpressionStmt(Box<Ast>),

    // Printing (Let's flex that output)
    PrintStmt(Box<Ast>),

    // Return (Endgame)
    ReturnStmt(Option<Box<Ast>>),

    // Binary operations (because life is full of memes)
    Binary {
        left: Box<Ast>,
        operator: BinaryOp, // Meme operator like `++` or `--`
        right: Box<Ast>,
    },

    // Unary operations (sometimes we just need that one meme)
    Unary {
        operator: BinaryOp,
        operand: Box<Ast>,
    },

    // Assignment (Assigning that grind)
    Assign {
        name: String,
        value: Box<Ast>,
    },

    // Variables (with epic memes)
    Variable(String),

    // Literals (simple and straight to the point)
    Literal(Value), // Literal values like numbers or strings

    // Grouping (wrapping memes in parentheses)
    Grouping(Box<Ast>), // Group those sus operations
}

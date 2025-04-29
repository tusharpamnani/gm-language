use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Skips
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*\n?", logos::skip)]

    // Error catch-all
    #[error]
    #[regex(r".", priority = 0)]
    Error,

    // Keywords for GM Language
    #[token("token")]
    Launch,      // 'Launch' maps to 'token' in the GM language
    #[token("block")]
    Debug,       // 'Debug' maps to 'block' in the GM language
    #[token("mine")]
    BossFight,   // 'BossFight' maps to 'mine' in the GM language
    #[token("ping")]
    Ping,        // 'Ping' maps to 'broadcast' in the GM language
    
    // Control flow
    #[token("if")]
    Sus,         // 'Sus' maps to 'if' in the GM language
    #[token("else")]
    Rekt,        // 'Rekt' maps to 'else' in the GM language
    #[token("while")]
    Grind,       // 'Grind' maps to 'while' in the GM language
    #[token("return")]
    GG,          // 'GG' maps to 'return' in the GM language
    #[token("loop")]
    Loop,        // 'Loop' keeps the name
    #[token("break")]
    Crash,       // 'Crash' maps to 'break' in the GM language
    
    // Types
    #[token("int")]
    TypeInt,
    #[token("str")]
    TypeStr,
    #[token("bool")]
    TypeBool,

    // Literals
    #[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse().ok())] 
    Number(f64),
    #[regex(r#""[^"]*""#, |lex| Some(String::from(&lex.slice()[1..lex.slice().len() - 1])))] 
    Text(String),
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Operators
    #[token("+")]
    Plus,        // Regular addition
    #[token("-")]
    Minus,       // Regular subtraction
    #[token("*")]
    Star,        // Regular multiplication
    #[token("/")]
    Slash,       // Regular division
    
    // Special GM operators
    #[token("stake")]
    StakeOp,     // Alternative for '+'
    #[token("yield")]
    YieldOp,     // Alternative for '*'
    #[token("burn")]
    BurnOp,      // Alternative for '-'
    #[token("swap")]
    SwapOp,      // Alternative for '/'
    
    // Comparison & Assignment
    #[token("==")]
    Equal,
    #[token("=")]
    Match,       // Assignment operator
    #[token("!=")]
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEqual,
    
    // Logical operators
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,

    // Identifier
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(String::from(lex.slice())))] 
    Identifier(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => write!(f, "error"),
            Token::Launch => write!(f, "token"),
            Token::Debug => write!(f, "block"),
            Token::BossFight => write!(f, "mine"),
            Token::Ping => write!(f, "broadcast"),
            Token::Sus => write!(f, "if"),
            Token::Rekt => write!(f, "else"),
            Token::Grind => write!(f, "while"),
            Token::GG => write!(f, "return"),
            Token::Loop => write!(f, "loop"),
            Token::Crash => write!(f, "break"),
            Token::TypeInt => write!(f, "int"),
            Token::TypeStr => write!(f, "str"),
            Token::TypeBool => write!(f, "bool"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Text(s) => write!(f, "\"{}\"", s),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::StakeOp => write!(f, "stake"),
            Token::YieldOp => write!(f, "yield"),
            Token::BurnOp => write!(f, "burn"),
            Token::SwapOp => write!(f, "swap"),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanEqual => write!(f, "<="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Arrow => write!(f, "->"),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Equal => write!(f, "=="),
            Token::Match => write!(f, "="),
            Token::NotEqual => write!(f, "!="),
        }
    }
}
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

    // Keywords
    #[token("launch")]
    Launch,
    #[token("loop")]
    Loop,
    #[token("crash")]
    Crash,
    #[token("debug")]
    Debug,
    #[token("rekt")]
    Rekt,
    #[token("sus")]
    Sus,
    #[token("gg")]
    GG,
    #[token("grind")]
    Grind,
    #[token("ping")]
    Ping,
    #[token("bossfight")]
    BossFight,

    // Types
    #[token("int")]
    TypeInt,
    #[token("str")]
    TypeStr,
    #[token("bool")]
    TypeBool,

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())] Number(i64),
    #[regex(r#""[^"]*""#, |lex| Some(String::from(&lex.slice()[1..lex.slice().len() - 1])))] Text(String),
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("==")]
    Equal,

    // Meme operators
    #[token("++")]
    Buff,
    #[token("--")]
    Nerf,
    #[token("wtf")]
    Wtf,
    #[token("lmao")]
    Lmao,
    #[token("match")]
    Match,
    #[token("carry")]
    Carry,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Comparison operators
    #[token("flex")] // >
    GreaterThan,
    #[token("grindset")] // >=
    GreaterThanEqual,
    #[token("cope")] // <
    LessThan,
    #[token("mald")] // <=
    LessThanEqual,
    #[token("ragequit")] // !=
    NotEqual,

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
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(String::from(lex.slice())))] Identifier(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => write!(f, "error"),
            Token::Launch => write!(f, "launch"),
            Token::Loop => write!(f, "loop"),
            Token::Crash => write!(f, "crash"),
            Token::Debug => write!(f, "debug"),
            Token::Rekt => write!(f, "rekt"),
            Token::Sus => write!(f, "sus"),
            Token::GG => write!(f, "gg"),
            Token::Grind => write!(f, "grind"),
            Token::Ping => write!(f, "ping"),
            Token::BossFight => write!(f, "bossfight"),
            Token::TypeInt => write!(f, "int"),
            Token::TypeStr => write!(f, "str"),
            Token::TypeBool => write!(f, "bool"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Text(s) => write!(f, "\"{}\"", s),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Buff => write!(f, "++"),
            Token::Nerf => write!(f, "--"),
            Token::Wtf => write!(f, "wtf"),
            Token::Lmao => write!(f, "lmao"),
            Token::Match => write!(f, "match"),
            Token::Carry => write!(f, "carry"),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::GreaterThan => write!(f, "flex"),
            Token::GreaterThanEqual => write!(f, "grindset"),
            Token::LessThan => write!(f, "cope"),
            Token::LessThanEqual => write!(f, "mald"),
            Token::NotEqual => write!(f, "ragequit"),
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
        }
    }
}

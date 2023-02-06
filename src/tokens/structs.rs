#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    
    And,
    Or,
    Not,

    Addition,
    Subtraction,
    Division,
    Multiplication,
    Exponent,
    Modulus,
    
    Assignment,
    
    Let,
    Print,
    If,

    Else,
    BeginBlock,
    EndBlock,
    EndStatement,

    LiteralString(String),
    LiteralNumber(usize),
    LiteralBoolean(bool),
    Variable(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrQual,
    
    And,
    Or,
    Not,

    Addition,
    Subtraction,
    Division,
    Multiplication,
    Exponent,
    Modulus,

    Let,
    Assignment,
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

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

    Let,
    Assignment,
    Print,
    If,
    Else,
    EndStatement,

    LiteralString(String),
    LiteralNumber(usize),
    LiteralBoolean(bool),
    Variable(String),
}

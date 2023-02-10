#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Boolean(bool),
    Number(usize),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    And,
    Or,

    Addition,
    Subtraction,
    Division,
    Multiplication,
    Exponent,
    Modulus,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    Block(Box<Block>),
    If {
        condition: Box<Expression>,
        if_branch: Box<Expression>,
        else_branch: Box<Option<Expression>>,
    },
    BinOp {
        op: BinaryOperator,
        left_side: Box<Expression>,
        right_side: Box<Expression>
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    Declaration {
        name: String,
        value: Box<Expression>,
    },
    Print(Box<Expression>),
}

pub type Block = Vec<Expression>;
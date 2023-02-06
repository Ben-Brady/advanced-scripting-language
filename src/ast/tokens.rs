#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Boolean,
    Number,
    String
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Number(usize),
    String(String),
    Boolean(bool),
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
    Print(Box<Expression>),
}

pub type AST = Vec<Expression>;
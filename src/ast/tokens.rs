#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Boolean,
    Number,
    String
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComparsionOperator {
    Equal,
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BooleanOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MathematicalOperator {
    Addition,
    Subtraction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Number(u64),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Literal(Literal),
    Variable(String),
    Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOperator {
    Comparison(ComparsionOperator),
    Boolean(BooleanOperator),
    Mathematical(MathematicalOperator),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AST {
    pub statements: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStatement {
    conditon: Expression,
    if_branch: Vec<Expression>,
    else_branch: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    If(Box<IfStatement>),
    BinOp(BinaryOperator),
    Value(Value),
}
use super::{Value, Type};
use crate::parser::BinaryOperator;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    VariableDoesNotExist(String),
    VariableAlreadyDeclared(String),
    UnsupportedOperation(BinaryOperator, Value, Value),
    InvalidType{
        expected: Type,
        actual: Value,
    },
}
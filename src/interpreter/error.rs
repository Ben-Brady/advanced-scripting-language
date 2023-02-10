
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    VariableDoesNotExist(String),
    VariableAlreadyDeclared(String),
}
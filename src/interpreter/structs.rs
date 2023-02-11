#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Number,
    Boolean,
    String,
    None
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Number(usize),
    Boolean(bool),
    String(String),
    None
}
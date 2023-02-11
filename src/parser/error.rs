#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnexpectedEndOfTokens,
    UnexpectedToken,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CouldntReadFile,
    UnexpectedEndOfTokens,
    InvalidToken,
    UnexpectedToken,
    InvalidType,
}

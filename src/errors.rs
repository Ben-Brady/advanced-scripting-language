
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CouldntReadFile,
    UnexpectedEndOfFile,
    UnexpectedEndOfTokens,
    InvalidToken,
    UnexpectedToken,
    UnexpectedKeyword,
}

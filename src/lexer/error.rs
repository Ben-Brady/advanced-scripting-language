#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CouldntReadFile,
    InvalidToken,
}
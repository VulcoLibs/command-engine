use super::*;
use std::error::Error as StdError;


pub type CEResult<T> = StdResult<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    DuplicatedCommandName(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Error::DuplicatedCommandName(name) => format!("Can't add a command with duplicated name [{}]", name),
            }
        )
    }
}

impl StdError for Error {}

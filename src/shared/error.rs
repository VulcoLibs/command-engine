// ToDo: Add comments

mod internal;

use super::*;
use std::io::ErrorKind;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;
pub use internal::InternalError;
#[cfg(feature = "json")] pub use serde_json::Error as JsonError;


pub type ResultCE<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    tag: Option<String>,
    pub source: Box<dyn std::error::Error>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            if let Some(tag) = &self.tag {
                format!("[{}]: {}", tag, self.source.to_string())
            } else {
                self.source.to_string()
            }
        )
    }
}

impl std::error::Error for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<internal::InternalError> for Error {
    fn from(internal_error: InternalError) -> Self {
        Self {
            tag: None,
            source: Box::new(internal_error)
        }
    }
}

impl From<tokio::io::Error> for Error {
    fn from(tokio_error: std::io::Error) -> Self {
        Self {
            tag: Some(String::from("Tokio")),
            source: Box::new(tokio_error)
        }
    }
}

impl <T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_send_error: SendError<T>) -> Self {
        InternalError::EngineIsNotRunning.into()
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_recv_error: RecvError) -> Self {
        InternalError::EngineIsNotRunning.into()
    }
}

impl Into<tokio::io::Error> for Error {
    fn into(self) -> std::io::Error {
        std::io::Error::new(ErrorKind::Other, self)
    }
}

use std::fmt::Display;
use super::*;


#[derive(Debug, Copy, Clone)]
pub enum InternalError {
    CommandAlreadyExists,
    EngineCapacityReached,
    EngineIsAlreadyRunning,
    EngineIsNotRunning,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InternalError {

}

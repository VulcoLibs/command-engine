// ToDo: Add tests

#[cfg(feature = "json")]
#[macro_use]
extern crate serde;

mod engine_sync;
mod engine_async;
mod instruction;
mod error;

pub use instruction::*;
pub use error::*;

#[cfg(test)]
mod tests;

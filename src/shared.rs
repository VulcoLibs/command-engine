use std::result::Result as StdResult;
use std::fmt::Result as FmtResult;
use std::collections::HashMap;
use std::fmt::{Formatter, Display};

mod instruction;
#[doc(hidden)] mod help;  // Undocumented, beta module
mod output;
#[doc(hidden)] mod errors;
#[doc(hidden)] pub mod commands;  // WIP

pub use instruction::*;
pub use help::{Help, SubArg};
pub use output::Output;
pub use errors::*;

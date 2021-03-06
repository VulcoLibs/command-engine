use std::result::Result as StdResult;
use std::fmt::Result as FmtResult;
use std::collections::HashMap;
use std::fmt::{Formatter, Display};

mod instruction;
#[doc(hidden)] mod help;  // Undocumented, beta module
mod output;
#[doc(hidden)] pub mod error;
#[doc(hidden)] mod commands;
#[doc(hidden)] mod traits;

#[macro_use]
mod macros;

pub use traits::*;
pub use instruction::*;
pub use help::{Help, SubArg};
pub use output::Output;

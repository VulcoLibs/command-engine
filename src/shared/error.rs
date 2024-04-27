use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    /// Instruction deserializer couldn't find Command caller in the input.
    ///
    /// Examples: <br>
    /// - `input = ""`
    /// - `input = "--o_arg"`
    /// - `input = "--o_arg caller"`
    InstructionMissingCaller,

    /// Instruction deserializer received input that had SubArg before OArg
    ///
    /// (this shouldn't happen).
    InstructionSubArgWithoutOArg,

    /// Engine couldn't find a suitable Command for the received caller.
    ///
    /// Example:
    /// ```pseudo
    /// Engine = {
    ///     commands: ["echo", "dir", "ls"]
    /// };
    ///
    /// Engine.execute("cd ..");  // Error(EngineCommandNotFound)
    /// ```
    EngineCommandNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

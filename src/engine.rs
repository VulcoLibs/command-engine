use super::*;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::{Error, Instruction};

/// Engine is the main part that can be implemented in any way.
///
/// This is a default Engine which acts as a container for the Commands which it can also execute
/// based on the raw input.
///
/// You can disable default features to ignore this default Engine implementation and create your own.
///
/// To use the async version enable `async` feature.
///
/// Example:
/// ```rust
/// let mut engine = Engine::new();
/// engine.add(/* your command here */);
///
/// let x = engine.execute(/* your input here */);
/// ```
#[repr(transparent)]
pub struct Engine<Output> {
    commands: HashMap<&'static str, Box<dyn Command<Output=Output>>>,
}

impl<Output: 'static> Engine<Output> {
    /// Creates a new empty Engine.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new Command.
    ///
    /// Each structure added need to implement `Command` trait and will be transformed into a trait object.
    ///
    /// If Command with the same caller already exists in the Engine it will be overwritten.
    pub fn insert<T: Command<Output=Output>>(&mut self, command: T) {
        let _ = self.commands.insert(
            command.caller(),
            Box::new(command)
        );
    }

    /// Removes Command based on its caller.
    ///
    /// If the Command was present in the Engine it will be returned.
    pub fn remove(&mut self, caller: impl AsRef<str>) -> Option<Box<dyn Command<Output=Output>>> {
        self.commands.remove(caller.as_ref())
    }

    /// Checks if there are any Commands in the Engine.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Based on the given `input` the Engine will choose related Command and trigger its `on_execute`
    /// method with the Instruction created from the input.
    ///
    /// This function can fail if the `input` is not in a valid Instruction format, or if Engine
    /// failed to find any related Command.
    #[cfg(feature = "async")]
    pub async fn execute(&self, input: impl AsRef<str>) -> Result<Output, Error> {
        let instruction = Instruction::new(input.as_ref())?;

        let command = self
            .commands
            .get(instruction.caller)
            .ok_or_else(|| Error::EngineCommandNotFound)?;

        let output = command.on_execute(instruction).await;
        Ok(output)
    }

    /// Based on the given `input` the Engine will choose related Command and trigger its `on_execute`
    /// method with the Instruction created from the input.
    ///
    /// This function can fail if the `input` is not in a valid Instruction format, or if Engine
    /// failed to find any related Command.
    #[cfg(not(feature = "async"))]
    pub fn execute(&self, input: impl AsRef<str>) -> Result<Output, Error> {
        let instruction = Instruction::new(input.as_ref())?;

        let command = self
            .commands
            .get(instruction.caller)
            .ok_or_else(|| Error::EngineCommandNotFound)?;

        let output = command.on_execute(instruction);
        Ok(output)
    }
}

impl<T> Default for Engine<T> {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
}

impl<T> Debug for Engine<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let entries = self
            .commands
            .iter()
            .map(|(caller, _)| *caller);

        fmt.debug_list().entries(entries).finish()
    }
}

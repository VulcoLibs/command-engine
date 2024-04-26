use super::*;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::{Error, Instruction};

#[repr(transparent)]
pub struct Engine<Output> {
    commands: HashMap<&'static str, Box<dyn Command<Output=Output>>>,
}

impl<Output: 'static> Engine<Output> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert<T: Command<Output=Output>>(&mut self, command: T) {
        let _ = self.commands.insert(
            command.caller(),
            Box::new(command)
        );
    }

    pub fn remove(&mut self, caller: impl AsRef<str>) -> Option<Box<dyn Command<Output=Output>>> {
        self.commands.remove(caller.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

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

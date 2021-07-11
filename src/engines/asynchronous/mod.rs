mod command;

use crate::shared::*;
use std::collections::HashMap;
pub use command::*;


/// Asynchronous Command Engine
pub struct AsyncEngine<'a> {
    commands: HashMap<String, Box<dyn AsyncCommand + 'a>>
}

impl<'a> AsyncEngine<'a> {
    /// Creates an empty AsyncEngine
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Box<dyn AsyncCommand + 'a>>::new()
        }
    }

    /// Adds a new Command to the AsyncEngine and returns itself
    ///
    /// # Arguments
    ///
    /// * `command_struct` - A struct with AsyncCommand trait implementation
    ///
    /// # Example
    /// ```
    /// use command_engine::{Output, Instruction};
    /// use command_engine::asynchronous::*;
    ///
    /// struct MyCommand;
    ///
    /// impl AsyncCommand for MyCommand {
    ///     (...)
    /// }
    ///
    /// let mut engine = AsyncEngine::new()
    ///   .add(MyCommand{});
    /// ```
    pub fn add<C: AsyncCommand + 'a>(mut self, command_struct: C) -> Self {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), Box::new(command_struct));
        }

        self
    }

    /// Adds a new Command to the AsyncEngine
    ///
    /// # Arguments
    ///
    /// * `command_struct` - A struct with AsyncCommand trait implementation
    ///
    /// # Example
    /// ```
    /// use command_engine::{Output, Instruction};
    /// use command_engine::asynchronous::*;
    ///
    /// struct MyCommand;
    ///
    /// impl AsyncCommand for MyCommand {
    ///     (...)
    /// }
    ///
    /// let mut engine = AsyncEngine::new();
    /// engine.add_separated(MyCommand{});
    /// ```
    pub fn add_separated<C: AsyncCommand + 'a>(&mut self, command_struct: C) {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), Box::new(command_struct));
        }
    }

    /// Gets a raw string, tries to convert it into an Instruction and tries to execute the AsyncCommand based on provided data
    ///
    /// # Arguments
    ///
    /// * `raw_instruction` - A string data with the command name and arguments
    ///
    /// # Example
    /// ```
    /// let raw = String::from("mycommand arg1 -o optional");
    ///
    /// let mut engine =
    /// (...)
    ///
    /// let output = engine.execute(&raw).await;
    /// println!("StatusCode: '{}'\n{}", output.result, output.message);
    /// ```
    pub async fn execute<S: AsRef<str>>(&mut self, raw_instruction: S) -> Output {
        let instruction = match Instruction::new(raw_instruction) {
            Ok(instruction) => instruction,
            Err(output) => return output,
        };

        let command = match self.get_command_mut(&instruction.value) {
            None => return Output::new_error(0, Some(String::from("Invalid command!"))),
            Some(command) => command,
        };

        if let Some(arg0) = instruction.args.get(0) {
            if arg0.eq("help") {
                return Output::new_ok(0, Some(command.on_help()));
            }
        }

        command.on_execute(&instruction).await
    }

    #[doc(hidden)]
    fn get_command(&self, name: &String) -> Option<&Box<dyn AsyncCommand + 'a>> {
        match self.commands.get(name) {
            None => None,
            Some(command) => Some(command),
        }
    }

    #[doc(hidden)]
    fn get_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn AsyncCommand + 'a>> {
        match self.commands.get_mut(name) {
            None => None,
            Some(command) => Some(command),
        }
    }
}

// Asynchronous Send and Sync implementations for AsyncEngine
unsafe impl<'a> Send for AsyncEngine<'a> {}
unsafe impl<'a> Sync for AsyncEngine<'a> {}

mod command;

use crate::shared::*;
use std::collections::HashMap;
pub use command::*;


/// Synchronous Command Engine
pub struct Engine<'a> {
    commands: HashMap<String, Box<dyn Command + 'a>>,
}

impl<'a> Engine<'a> {
    /// Creates an empty Engine
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Box<dyn Command  + 'a>>::new(),
        }
    }

    /// Adds a new Command to the Engine and returns itself
    ///
    /// # Arguments
    ///
    /// * `command_struct` - A struct with Command trait implementation
    ///
    /// # Example
    /// ```
    /// use command_engine::*;
    ///
    /// struct MyCommand;
    ///
    /// impl Command for MyCommand {
    ///     (...)
    /// }
    ///
    /// let mut engine = Engine::new()
    ///   .add(MyCommand{});
    /// ```
    pub fn add<C: Command + 'a>(mut self, command_struct: C) -> Self {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), Box::new(command_struct));
        }

        self
    }

    /// Adds a new Command to the Engine
    ///
    /// # Arguments
    ///
    /// * `command_struct` - A struct with Command trait implementation
    ///
    /// # Example
    /// ```
    /// use command_engine::*;
    ///
    /// struct MyCommand;
    ///
    /// impl Command for MyCommand {
    ///     (...)
    /// }
    ///
    /// let mut engine = Engine::new();
    /// engine.add_separated(MyCommand{});
    /// ```
    pub fn add_separated<C: Command + 'a>(&mut self, command_struct: C) {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), Box::new(command_struct));
        }
    }

    /// Gets a raw string, tries to convert it into an Instruction and tries to execute the Command based on provided data
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
    /// let output = engine.execute(&raw);
    /// println!("StatusCode: '{}'\n{}", output.result, output.message);
    /// ```
    pub fn execute<S: AsRef<str>>(&mut self, raw_instruction: S) -> Output {
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

        command.on_execute(&instruction)
    }

    #[doc(hidden)]
    fn get_command(&self, name: &String) -> Option<&Box<dyn Command + 'a>> {
        match self.commands.get(name) {
            None => None,
            Some(command) => Some(command),
        }
    }

    #[doc(hidden)]
    fn get_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn Command + 'a>> {
        match self.commands.get_mut(name) {
            None => None,
            Some(command) => Some(command),
        }
    }
}

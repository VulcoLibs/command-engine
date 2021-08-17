use super::*;

/// Trait allowing the struct to be used as a Command in the synchronous Engine
pub trait Command {
    /// Returns name of the Command by which it will be detected
    fn name(&self) -> &str;

    /// Returns help message when `<CommandName> help` was called
    #[allow(unused_variables)]
    fn on_help(&self, ins: &Instruction) -> String {
        Help::new((&self).name(), "help is not implemented for this sync command").format_compact()
    }

    /// Logic that executes when the Instruction had the Command's name
    ///
    /// Returns an Output indicating in the success or failure of the Command execution
    fn on_execute(&self, ins: &Instruction) -> Output;
}

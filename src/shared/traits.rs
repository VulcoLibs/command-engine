use super::*;


pub trait CommandInfo {
    fn caller(&self) -> &str;

    #[allow(unused_variables)]
    fn on_help(&self, ins: Instruction) -> String {
        Help::new((&self).caller(), "help is not implemented for this command").format_compact()
    }
}

#[async_trait::async_trait]
pub trait Command: CommandInfo + Send + Sync {
    async fn on_execute(&self, ins: Instruction) -> Output;
}

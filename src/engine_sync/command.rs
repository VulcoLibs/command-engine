use crate::{CommandInfo, Instruction};

pub trait Command: CommandInfo + 'static {
    type Output;

    fn on_execute(&self, ins: Instruction) -> Self::Output;
}

#![allow(unused_imports)]

mod messages;

#[cfg(feature = "c-sys")]
pub mod sys;

#[cfg(feature = "c-fs")]
pub mod fs;

#[cfg(feature = "async")]
use crate::engines::AsyncCommand;
use crate::engines::Command;
use super::macros::residual;

use crate::shared::{
    Help,
    Output,
    Instruction,
};

use macros::autoimpl_async_command;


#[allow(unused_macros)]
#[macro_use]
mod macros {
    #![allow(unused_imports)]
    macro_rules! autoimpl_async_command {
        ($name:ident) => {
            #[cfg(feature = "async")]
            #[async_trait]
            impl AsyncCommand for $name {
                fn name(&self) -> &str {
                    Command::name(self)
                }

                fn on_help(&self, ins: &Instruction) -> String {
                    Command::on_help(self, ins)
                }

                async fn on_execute(&self, ins: &Instruction) -> Output {
                    Command::on_execute(self, ins)
                }
            }
        };
    }

    pub (crate) use autoimpl_async_command;
}

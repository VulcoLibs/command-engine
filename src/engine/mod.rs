mod builder;
mod directive;
#[doc(hidden)] mod config;

use directive::Directive;
use tokio::sync::{mpsc, oneshot};
pub use builder::EngineBuilder;
use crate::shared::*;
use error::*;
use super::Command;


/// Engine communicator. Used to send directives to the Engine.
///
/// Can be cloned cheaply, because it's only a wrapper over tokio's mpsc sender.
///
/// Every method returns a Result, because in every case there can be an issue where the Engine is disabled.
/// Err variant always refer to the Engine and not the communicator.
///
#[derive(Clone)]
pub struct Engine {
    sender: mpsc::Sender<Directive>,
}

impl Engine {
    /// Adds a new Command to the Engine.
    ///
    /// Returns error if a Command with the same caller already exists.
    ///
    /// # Arguments
    ///
    /// * `command` - A struct implementing Command trait
    ///
    pub async fn add<C: Command + 'static>(&self, command: C) -> ResultCE<()> {
        let (tx, rx) = oneshot::channel();

        self.sender.send(Directive::CommandAdd {
            command: Box::new(command),
            resp: tx,
        }).await?;

        let result = rx.await?;
        result
    }

    /// Removes a Command from the Engine.
    ///
    /// Returns result which doesn't tell if the command was removed or not.
    /// If it exists, it will always be removed.
    ///
    /// # Arguments
    ///
    /// * `caller` - A Command caller
    ///
    pub async fn remove<S: ToString>(&self, caller: S) -> ResultCE<()> {
        self.sender.send(Directive::CommandRemove {
            caller: caller.to_string()
        }).await?;

        Ok(())
    }

    /// Executes a given input.
    ///
    /// Returns an Output based on the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - A String representing raw Instruction to execute
    ///
    pub async fn execute<S: ToString>(&self, input: S) -> ResultCE<Output> {
        let (tx, rx) = oneshot::channel();

        self.sender.send(Directive::Execute {
            input: input.to_string(),
            resp: tx,
        }).await?;

        // let result = rx.await?;
        // Ok(result)

        match rx.await? {
            Ok((command, instruction)) => {
                Ok(command.on_execute(instruction).await)
            }
            Err(output) => {
                Ok(output)
            }
        }
    }

    /// Closes the Engine.
    ///
    pub async fn close(self) -> ResultCE<()> {
        self.sender.send(Directive::Close).await?;
        Ok(())
    }
}

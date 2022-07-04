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

    /// Same as `add()` method
    pub fn add_blocking<C: Command + 'static>(&self, command: C) -> ResultCE<()> {
        let (tx, rx) = oneshot::channel();

        self.sender.blocking_send(Directive::CommandAdd {
            command: Box::new(command),
            resp: tx,
        })?;

        rx.blocking_recv()?
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

    /// Same as `remove` method
    pub fn remove_blocking<S: ToString>(&self, caller: S) -> ResultCE<()> {
        self.sender.blocking_send(Directive::CommandRemove {
            caller: caller.to_string()
        })?;

        Ok(())
    }

    async fn _execute<S: ToString, C: ToString>(&self, input: S, on_caller: Option<C>) -> ResultCE<Output> {
        let (tx, rx) = oneshot::channel();

        self.sender.send(Directive::Execute {
            input: input.to_string(),
            on_caller: on_caller.map(|c| c.to_string()),
            resp: tx,
        }).await?;

        match rx.await? {
            Ok((command, instruction)) => {
                Ok(command.on_execute(instruction).await)
            }
            Err(output) => {
                Ok(output)
            }
        }
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
        self._execute::<_, String>(input, None).await
    }

    /// Same as `execute`, but runs only on a specific caller.
    ///
    /// # Arguments
    ///
    /// * `input` - A String representing raw Instruction to execute
    /// * `caller` - A command's caller to execute on
    ///
    pub async fn execute_on_caller(&self, input: impl ToString, caller: impl ToString) -> ResultCE<Output> {
        self._execute(input, Some(caller)).await
    }

    /// Closes the Engine.
    ///
    pub async fn close(self) -> ResultCE<()> {
        self.sender.send(Directive::Close).await?;
        Ok(())
    }

    /// Checks if a specific command exists
    ///
    pub async fn contains(&self, caller: impl ToString) -> ResultCE<bool> {
        let (tx, rx) = oneshot::channel();

        self.sender.send(Directive::Exists {
            caller: caller.to_string(),
            resp: tx,
        }).await?;

        Ok(rx.await?)
    }

    /// Same as `contains()`
    ///
    pub fn contains_blocking(&self, caller: impl ToString) -> ResultCE<bool> {
        let (tx, rx) = oneshot::channel();

        self.sender.blocking_send(Directive::Exists {
            caller: caller.to_string(),
            resp: tx,
        })?;

        Ok(rx.blocking_recv()?)
    }
}

use super::*;

/// Each type implementing `Command` trait must also implement `CommandInfo` which is used only to
/// retrieve information about the Command.
pub trait CommandInfo {
    fn caller(&self) -> &'static str;
}

#[cfg(feature = "async")]
mod command_async {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Example:
    /// ```rust
    /// use command_engine::{Command, CommandInfo, Instruction, OutputFuture, IntoOutputFuture};
    ///
    /// struct MyCommand1;
    ///
    /// impl CommandInfo for MyCommand1 {
    ///     fn caller(&self) -> &'static str {
    ///         "command1"
    ///     }
    /// }
    ///
    /// impl Command for MyCommand1 {
    ///     type Output = ();
    ///
    ///     fn on_execute<'a>(&self, _ins: Instruction<'a>) -> OutputFuture<'a, Self::Output> {
    ///         async move {
    ///             ()
    ///         }.output_future()
    ///     }
    /// }
    /// ```
    pub trait Command: CommandInfo + Send + Sync + 'static {
        type Output;

        fn on_execute<'a>(&self, ins: Instruction<'a>) -> OutputFuture<'a, Self::Output>;
    }

    pub struct OutputFuture<'a, Output> {
        future: Pin<Box<dyn Future<Output=Output> + Send + 'a>>,
    }

    impl<'a, Output> Future for OutputFuture<'a, Output> {
        type Output = Output;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.future.as_mut().poll(cx)
        }
    }

    pub trait IntoOutputFuture<'a, Output> {
        fn output_future(self) -> OutputFuture<'a, Output>;
    }

    impl<'a, F: Future<Output=Output> + Send + 'a, Output> IntoOutputFuture<'a, Output> for F {
        fn output_future(self) -> OutputFuture<'a, Output> {
            OutputFuture {
                future: Box::pin(self),
            }
        }
    }
}

#[cfg(not(feature = "async"))]
mod command_sync {
    use super::*;

    /// Example:
    /// ```rust
    /// use command_engine::{Command, CommandInfo, Instruction};
    ///
    /// struct MyCommand1;
    ///
    /// impl CommandInfo for MyCommand1 {
    ///     fn caller(&self) -> &'static str {
    ///         "command1"
    ///     }
    /// }
    ///
    /// impl Command for MyCommand1 {
    ///     type Output = ();
    ///
    ///     fn on_execute(&self, _ins: Instruction) -> Self::Output {
    ///         ()
    ///     }
    /// }
    /// ```
    pub trait Command: CommandInfo + 'static {
        type Output;

        fn on_execute(&self, ins: Instruction) -> Self::Output;
    }
}

#[cfg(feature = "async")]
pub use command_async::*;

#[cfg(not(feature = "async"))]
pub use command_sync::*;


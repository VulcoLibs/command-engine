use super::*;

pub trait CommandInfo {
    fn caller(&self) -> &'static str;
}

#[cfg(feature = "async")]
mod command_async {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

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

    pub trait Command: CommandInfo + 'static {
        type Output;

        fn on_execute(&self, ins: Instruction) -> Self::Output;
    }
}

#[cfg(feature = "async")]
pub use command_async::*;

#[cfg(not(feature = "async"))]
pub use command_sync::*;


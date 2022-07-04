// ToDo: Add comments

use std::sync::Arc;
use crate::Command;
use crate::shared::error::*;
use crate::shared::{Instruction, Output};


pub type Resp<T> = tokio::sync::oneshot::Sender<T>;

pub(crate) enum Directive {
    CommandAdd {
        command: Box<dyn Command>,
        resp: Resp<ResultCE<()>>,
    },
    CommandRemove {
        caller: String,
    },
    Execute {
        input: String,
        on_caller: Option<String>,
        resp: Resp<Result<(Arc<Box<dyn Command>>, Instruction), Output>>,
    },
    Exists {
        caller: String,
        resp: Resp<bool>
    },
    Close,
}

unsafe impl Sync for Directive {}
unsafe impl Send for Directive {}

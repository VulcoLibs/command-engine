use super::*;
use std::path::Path;
use std::env::{current_dir, set_current_dir};


pub struct Cwd;

impl Cwd {
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }

    fn execute(&mut self, ins: &Instruction) -> Output {
        if ins.args.len() < 1 {
            return match current_dir() {
                Ok(path) => {
                    Output::new_ok(1, Some(path.display()))
                }
                Err(error) => {
                    Output::new_error(1, Some(error.to_string()))
                }
            }
        }

        let path = Path::new(ins.args.get(1).unwrap());

        if !(path.exists() && path.is_dir()) {
            return Output::new_error(2, Some(messages::ERR_NOT_A_DIR_PATH));
        }

        return match set_current_dir(&path) {
            Ok(_) => {
                Output::new_ok(2, Some(path.display()))
            }
            Err(error) => {
                Output::new_error(3, Some(error.to_string()))
            }
        }
    }
}

impl Command for Cwd {
    fn name(&self) -> &str {
        "cwd"
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output {
        self.execute(ins)
    }
}

autoimpl_async_command!(Cwd);

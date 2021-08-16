use super::*;
use std::env;


#[must_use = "Correct functionality of this command isn't guaranteed as it wasn't tested yet!"]
pub struct Env {

}

impl Env {
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }

    fn execute(&mut self, ins: &Instruction) -> Output {
        if ins.args.len() < 2 {
            return Output::new_error(1, Some(messages::ERR_ARGUMENTS_NUM));
        }

        let key = ins.args.get(1).unwrap();

        return match ins.args.get(0).unwrap().to_lowercase().as_ref() {
            "get" => {
                match env::var(key) {
                    Ok(res) => Output::new_ok(1, Some(res)),
                    Err(error) => Output::new_error(3, Some(error.to_string())),
                }
            }

            "set" => {
                if ins.args.len() < 3 {
                    return Output::new_error(1, Some(messages::ERR_ARGUMENTS_NUM));
                }

                let val = ins.args.get(2).unwrap();

                env::set_var(key, val);

                Output::new_ok(1, Some(format!("{}: {}", key, val)))
            }

            op => {
                Output::new_error(2, Some(format!("invalid operation [{}]", op)))
            }
        };
    }
}

impl Command for Env {
    fn name(&self) -> &str {
        "env"
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output {
        self.execute(ins)
    }
}

autoimpl_async_command!(Env);

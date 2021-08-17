use super::*;
use std::path::Path;
use tokio::fs::copy as copy_async;
use std::fs::copy as copy_sync;
use std::io::Error;



/// Soon DIR copy actions
enum CpAction {
    FileNew,
    FileOver,
}


#[must_use = "Correct functionality of this command isn't guaranteed as it wasn't tested yet!"]
pub struct Cp;

impl Cp {
    pub const fn name() -> &'static str {
        "cp"
    }

    pub const fn new() -> Self {
        Self {

        }
    }

    pub async fn copy_file_async<PSource: AsRef<Path>, PTarget: AsRef<Path>>(source: PSource, target: PTarget) -> Output {
        if let Err(error) = copy_async(source, target).await {
            Output::new_error(4, Some(error))
        } else {
            Output::new_ok::<Error>(1, None)
        }
    }

    pub fn copy_file_sync<PSource: AsRef<Path>, PTarget: AsRef<Path>>(source: PSource, target: PTarget) -> Output {
        if let Err(error) = copy_sync(source, target) {
            Output::new_error(4, Some(error))
        } else {
            Output::new_ok::<Error>(1, None)
        }
    }

    fn parse_instruction(ins: &Instruction) -> Result<(&Path, &Path, CpAction), Output> {
        if ins.args.len() < 2 {
            return Err(Output::new_error(1, Some(messages::ERR_ARGUMENTS_NUM)));
        }

        let source = Path::new(ins.args.get(0).unwrap());
        let target = Path::new(ins.args.get(1).unwrap());

        if source.is_file() {
            if target.exists() {
                if ins.oargs.contains_key("-o") {
                    return Ok((source, target, CpAction::FileOver));
                }

                return Err(Output::new_error(3, Some("can't copy override without oarg -o")));
            }

            return Ok((source, target, CpAction::FileNew));
        }

        Err(Output::new_error(2, Some("both args have to be valid file paths")))
    }
}

impl Command for Cp {
    fn name(&self) -> &str {
        Self::name()
    }

    fn on_execute(&self, ins: &Instruction) -> Output {
        let (source, target, action) = residual!(Self::parse_instruction(ins));

        return match action {
            CpAction::FileNew | CpAction::FileOver => {
                Self::copy_file_sync(source, target)
            }
        };
    }
}

#[async_trait]
impl AsyncCommand for Cp {
    fn name(&self) -> &str {
        Self::name()
    }

    async fn on_execute(&self, ins: &Instruction) -> Output {
        let (source, target, action) = residual!(Self::parse_instruction(ins));

        return match action {
            CpAction::FileNew | CpAction::FileOver => {
                Self::copy_file_async(source, target).await
            }
        };
    }
}

// ToDo: Add comments

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use super::*;
use config::EngineConfig;


lazy_static! {
    static ref OPERATIONAL_AB: AtomicBool = {
        AtomicBool::new(false)
    };
}

type CommandsMap = HashMap::<String, Arc<Box<dyn Command + 'static>>>;


#[derive(Clone, Debug)]
pub struct EngineBuilder {
    buffer: usize,
    config: EngineConfig,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            buffer: 32,
            config: EngineConfig::default(),
        }
    }
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }

    pub fn help_caller<S: ToString>(mut self, help_caller: S) -> Self {
        self.config.help_caller = help_caller.to_string();
        self
    }

    async fn run(mut tx: mpsc::Receiver<Directive>, config: EngineConfig) {
        let mut run = true;
        let mut commands = CommandsMap::new();

        while let Some(directive) = tx.recv().await {
            use Directive::*;

            match directive {

                CommandAdd { command, resp } => {
                    let result = {
                        let name = command.caller();
                        if commands.contains_key(name) || name == &config.help_caller {
                            Err(InternalError::CommandAlreadyExists.into())
                        } else {
                            Ok(())
                        }
                    };

                    if result.is_ok() {
                       commands.insert(command.caller().to_string(), Arc::new(command));
                    }

                    run = resp.send(result).is_ok();
                }

                CommandRemove { caller } => {
                    drop(commands.remove(&caller));
                }

                Execute { input, resp } => {
                    let instruction = match Instruction::new(input) {
                        Ok(instruction) => instruction,
                        Err(output) => {
                            run = resp.send(Err(output)).is_ok();
                            continue;
                        }
                    };

                    let command = match commands.get(instruction.get_caller()) {
                        None => {
                            run = resp.send(Err(Output::new_error(0, Some("Invalid command!")))).is_ok();
                            continue;
                        },
                        Some(command) => command.clone(),
                    };

                    if let Some(arg0) = instruction.get_args().get(0) {
                        if arg0 == &config.help_caller {
                            run = resp.send(Err(Output::new_ok(0, Some(command.on_help(instruction))))).is_ok();
                            continue;
                        }
                    }

                    run = resp.send(Ok(
                        (
                            command,
                            instruction,
                        )
                    )).is_ok();
                }

                Close => {
                    println!("Calling to die");
                    tx.close();
                    run = false;
                }

            }

            if run == false {
                break;
            }

        }

        println!("DEAD!");
    }

    fn mark_operational(value: bool) {
        // ToDo: Change to weaker atomic ordering
        OPERATIONAL_AB.store(value, Ordering::SeqCst)
    }

    pub fn is_running() -> bool {
        // ToDo: Change to weaker atomic ordering
        OPERATIONAL_AB.load(Ordering::SeqCst)
    }

    // pub fn connect() -> Option<Engine> {
    //     if Self::is_running() {
    //
    //     }
    // }

    pub fn build(self) -> ResultCE<Engine> {
        if Self::is_running() {
            return Err(InternalError::EngineIsAlreadyRunning.into());
        }

        let (tx, rx) = mpsc::channel::<Directive>(self.buffer);
        let config = self.config;

        tokio::spawn(async move {
            Self::run(rx, config).await;
            Self::mark_operational(false);
        });

        let engine = Engine {
            sender: tx,
        };

        Ok(engine)
    }
}

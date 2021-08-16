use super::*;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use tokio::fs::read_dir;
use std::io::Error;
use tokio::runtime;



#[derive(Eq, PartialEq)]
enum LdAction {
    ALL,
    FILES,
    DIRS,
}


#[must_use = "Correct functionality of this command isn't guaranteed as it wasn't tested yet!"]
pub struct Ld;

impl Ld {
    const fn name() -> &'static str {
        "ld"
    }

    pub const fn new() -> Self {
        Self {

        }
    }

    async fn format_path_content<P: AsRef<Path>>(path: P, action: LdAction) -> String {
        let mut dir_data = Vec::<String>::new();
        let mut file_data = Vec::<String>::new();

        if action == LdAction::FILES || action == LdAction::ALL {
            file_data.push(String::from("--- Files ---\n"));
        }

        if action == LdAction::DIRS || action == LdAction::ALL {
            dir_data.push(String::from("--- Dirs ---\n"));
        }

        let stream = read_dir(path.as_ref()).await;
        for mut rd in stream {
            if let Some(dir_entry) = rd.next_entry().await.unwrap_or(None) {
                let entry_path = dir_entry.path();

                if entry_path.is_dir() {
                    if action == LdAction::DIRS || action == LdAction::ALL {
                        dir_data.push(format!(
                            "{}\n",
                            dir_entry.file_name().to_string_lossy()
                        ));
                    }
                } else if entry_path.is_file() {
                    if action == LdAction::FILES || action == LdAction::ALL {
                        file_data.push(format!(
                            "{}\n",
                            dir_entry.file_name().to_string_lossy()
                        ));
                    }
                }
            }
        }

        let mut data = String::new();

        for x in dir_data {
            data.push_str(&*x)
        }

        for x in file_data {
            data.push_str(&*x)
        }

        data
    }

    async fn execute(&mut self, ins: &Instruction) -> Output {
        let get_action = || {
            match {
                (
                    ins.oargs.get("-f") != None,
                    ins.oargs.get("-d") != None,
                )
            } {
                (true, true) | (false, false) => LdAction::ALL,
                (true, false) => LdAction::FILES,
                (false, true) => LdAction::DIRS,
            }
        };

        if ins.args.is_empty() {
            return match current_dir() {
                Ok(path) => Output::new_ok(1, Some(Self::format_path_content(&*path, get_action()).await)),
                Err(error) => Output::new_error(1, Some(error)),
            }
        }

        let dir = Path::new(ins.args.get(0).unwrap());

        return if dir.exists() && dir.is_dir() {
            Output::new_ok(1, Some(Self::format_path_content(dir, get_action()).await))
        } else {
            Output::new_error(2, Some(messages::ERR_NOT_A_DIR_PATH))
        }
    }
}

impl Command for Ld {
    fn name(&self) -> &str {
        Ld::name()
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output {
        let future = async move {
            self.execute(ins).await
        };

        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }
}

#[async_trait]
impl AsyncCommand for Ld {
    fn name(&self) -> &str {
        Ld::name()
    }

    async fn on_execute(&mut self, ins: &Instruction) -> Output {
        self.execute(ins).await
    }
}

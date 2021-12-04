use super::*;


/// Instruction wrapper for Command name, arguments and optional arguments.
/// Created from string.
/// Used by the synchronous and asynchronous Engine.
#[derive(Debug)]
pub struct Instruction {
    caller: String,
    args: Vec<String>,
    oargs: HashMap<String, Option<String>>,
}

impl Instruction {
    // ToDo: Rewrite the parser
    fn parser(raw: String) -> Vec<String> {
        let mut commands = Vec::<String>::new();

        let mut fake_split = false;
        let mut spaceable = false;
        let mut tmp = String::new();
        for ch in raw.chars().into_iter() {
            if ch == '"' && !fake_split {
                spaceable = !spaceable;

                if tmp.is_empty() {
                    continue;
                }
                commands.push(tmp.clone());
                tmp.clear();

                continue;
            }

            if fake_split {
                fake_split = false
            } else if ch == '\\' {
                fake_split = true;
                continue;
            }

            if spaceable {
                tmp.push(ch);
            } else {
                if ch == ' ' {

                    if tmp.is_empty() {
                        continue;
                    }
                    commands.push(tmp.clone());
                    tmp.clear();

                    continue;
                } else {
                    tmp.push(ch);
                }
            }
        }

        if !tmp.is_empty() {
            commands.push(tmp);
        }

        commands
    }

    pub fn new<S: ToString>(input: S) -> StdResult<Self, Output> where Self: Sized {
        use std::mem;

        // let raw = match String::from_utf8(input.into_vec()) {
        //     Ok(raw) => raw,
        //     Err(error) => {
        //         return Err(Output::new_error(0, Some(error)));
        //     }
        // };

        let raw = input.to_string();

        let mut parts = Self::parser(raw);

        if parts.is_empty() {
            return Err(Output::new_error(0, Some("Invalid instruction!")));
        }

        let caller = unsafe{ mem::take(parts.get_unchecked_mut(0)) };
        let mut args = Vec::<String>::new();
        let mut oargs = HashMap::<String, Option<String>>::new();

        let mut tmp_key = String::new();
        let mut waiting_for_val = false;
        for part in &mut parts[1..] {
            match (part.starts_with("-"), waiting_for_val) {
                (true, false) => {
                    tmp_key = mem::take(part);
                    waiting_for_val = true;
                },
                (true, true) => {
                    oargs.insert(mem::take(&mut tmp_key), None);
                    tmp_key = mem::take(part);
                },
                (false, true) => {
                    oargs.insert(mem::take(&mut tmp_key), Some(mem::take(part)));
                    waiting_for_val = false;
                },
                (false, false) => {
                    args.push(mem::take(part));
                },
            }
        }

        if !tmp_key.is_empty() {
            oargs.insert(tmp_key, None);
        }

        Ok (Self {
            caller,
            args,
            oargs
        } )
    }

    pub fn get_caller(&self) -> &String {
        &self.caller
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_oargs(&self) -> &HashMap<String, Option<String>> {
        &self.oargs
    }
}

/// Display trait implementation for Instruction struct showing all the attributes.
///
/// * Command name
///
/// * Command arguments
///
/// * Command optional arguments
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Command: [\n\t{}\n]\nArgs: [\n\t{:?}\n]\nOargs: [\n\t{:?}\n]",
            &self.caller,
            &self.args,
            &self.oargs,
        )
    }
}

use super::*;


#[doc(hidden)]
fn parser<S: AsRef<str>>(s: S) -> Vec<String> {
    let raw = s.as_ref();
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


/// Instruction wrapper for Command name, arguments and optional arguments.
/// Created from string.
/// Used by the synchronous and asynchronous Engine.
#[derive(Default, Debug)]
pub struct Instruction {
    pub value: String,
    pub args: Vec<String>,
    pub oargs: HashMap<String, Option<String>>,
}

impl Instruction {
    /// Creates an Instruction object from provided string
    ///
    /// # Arguments
    ///
    /// * `raw` - A string data with the command name and arguments
    pub fn new<S: AsRef<str>>(s: S) -> StdResult<Self, Output> {
        let raw = s.as_ref();
        let commands = parser(raw);

        if commands.is_empty() {
            return Err(
                Output::new_error(0, Some(String::from("Invalid instruction!")))
            )
        }

        let value = commands.get(0).unwrap().clone();
        let mut args = Vec::<String>::new();
        let mut oargs = HashMap::<String, Option<String>>::new();

        let mut tmp_key = String::new();
        let mut is_flag = false;
        for x in &commands[1..] {
            if x.starts_with("-") {
                if is_flag {
                    oargs.insert(tmp_key.clone(), None);
                } else {
                    is_flag = true;
                }

                tmp_key.clear();
                tmp_key = x.clone();
                continue;
            }

            if is_flag {
                is_flag = false;
                oargs.insert(tmp_key.clone(), Some(x.clone()));
                tmp_key.clear();
                continue;
            }

            args.push(x.clone());
        }

        if !tmp_key.is_empty() {
            oargs.insert(tmp_key.clone(), None);
        }

        Ok( Self {
            value,
            args,
            oargs,
        } )
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
            &self.value,
            &self.args,
            &self.oargs,
        )
    }
}

use super::*;


/// Parses description into structural-friendly style.
///
/// # Arguments
/// * `s` - A generic with `AsRef<str>` implementation
///
/// Returns formatted `String`.
fn parse_desc<S: AsRef<str>>(s: S) -> String {
    let mut desc = s.as_ref().to_string();

    if desc.is_empty() {
        return String::new();
    }

    if desc.ends_with("\n") {
        desc.pop();
    }

    desc = desc.replace("\t", "");
    desc = desc.replace("\n", "\n\t\t");

    return desc;
}


/// Structure for representing positional arguments.
#[derive(Clone, Default, Debug)]
struct Args {
    /// `Key` - Name of the positional argument <br>
    /// `Value` - Description of the positional argument
    val: HashMap<String, String>
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut display = String::from("ARGS:\n");

        for (key, value) in self.val.iter() {
            display += &*format!("\t[{}] - {}\n", key, parse_desc(value))
        }

        display.pop();

        write!(
            f,
            "{}",
            display,
        )
    }
}


/// Structure for representing optional arguments.
#[derive(Clone, Default, Debug)]
struct OArgs {
    /// `Key` - Name of the optional argument <br>
    /// `Value.0` - Description of the optional argument <br>
    /// `Value.1` - Sub argument (name, is_optional)
    val: HashMap<String, (String, Option<(String, bool)>)>
}

impl Display for OArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut display = String::from("OARGS:\n");

        for (key, (desc, sub_arg)) in self.val.iter() {
            let key_parsed = if sub_arg.is_some() {
                let (name, is_optional) = sub_arg.as_ref().unwrap();
                if *is_optional {
                    format!("{} [{}]", key, name)
                } else {
                    format!("{} ({})", key, name)
                }
            } else {
                key.clone()
            };

            display += &*format!("\t[{}] - {}\n", key_parsed, parse_desc(desc))
        }

        display.pop();

        write!(
            f,
            "{}",
            display,
        )
    }
}


/// Structure for representing sub arguments.
#[derive(Debug, Clone)]
pub struct SubArg {
    pub name: String,
    pub is_optional: bool,
}

impl SubArg {
    pub fn new<S: ToString>(name: S, is_optional: bool) -> Self {
        Self {
            name: name.to_string(),
            is_optional,
        }
    }
}


/// Structure for easier "help" creation.
#[derive(Clone)]
pub struct Help {
    caller: String,
    desc: String,
    args: Args,
    oargs: OArgs,
}

impl Help {
    /// Creates new, simple `Help` instance.
    ///
    /// # Arguments
    /// * `name` - Name of the command.
    /// * `desc` - Command description.
    ///
    /// Returns `Help`.
    pub fn new<T: ToString, U: ToString>(name: T, desc: U) -> Self {
        Self {
            caller: name.to_string(),
            desc: desc.to_string(),
            args: Default::default(),
            oargs: Default::default(),
        }
    }

    /// Adds an argument into the Help.
    ///
    /// # Arguments
    /// * `name` - Name of the argument.
    /// * `desc` - Argument description.
    ///
    /// Returns `Help`.
    ///
    /// Example:
    /// ```rust
    /// let help = Help::new("mycommand", "an empty tut command")
    ///     .add_arg("argument1", "does nothing");
    /// ```
    pub fn add_arg<T: ToString, U: ToString>(mut self, name: T, desc: U) -> Self {
        self.args.val.insert(
            name.to_string(),
            desc.to_string()
        );

        self
    }

    /// Adds an optional argument into the Help.
    ///
    /// # Arguments
    /// * `name` - Name of the o-argument.
    /// * `desc` - O-Argument description.
    /// * `sub_arg` - Sub argument representation of the optional argument.
    ///
    /// Returns `Help`.
    pub fn add_oarg<T: ToString, U: ToString>(mut self, name: T, desc: U, sub_arg: Option<SubArg>) -> Self {
        let sub_arg_parsed = if sub_arg.is_some() {
            let sub_arg_unwrapped = sub_arg.unwrap();

            Some((
                sub_arg_unwrapped.name,
                sub_arg_unwrapped.is_optional,
            ))
        } else {
            None
        };

        self.oargs.val.insert(
            name.to_string(),
            (desc.to_string(), sub_arg_parsed)
        );

        self
    }

    /// Gets the command name that the `Help` is referring to.
    pub fn get_command_name(&self) -> &str {
        self.caller.as_str()
    }

    /// Gets the command description that the `Help` is referring to.
    pub fn get_command_description(&self) -> &str {
        self.desc.as_str()
    }

    /// Formats the `Help` into a structural style.
    pub fn format_structure(&self) -> String {
        match (
            self.args.val.is_empty(),
            self.oargs.val.is_empty()
        ) {
            (true, true) => {
                format!("[{}] - {}", self.caller, parse_desc(&self.desc))
            }
            (false, true) => {
                format!("NAME:\n\t[{}] - {}\n\n{}\n", self.caller, parse_desc(&self.desc), self.args)
            }
            (true, false) => {
                format!("NAME:\n\t[{}] - {}\n\n{}\n", self.caller, parse_desc(&self.desc), self.oargs)
            }
            (false, false) => {
                format!("NAME:\n\t[{}] - {}\n\n{}\n\n{}\n", self.caller, parse_desc(&self.desc), self.args, self.oargs)
            }
        }
    }

    /// Formats the `Help` into a compact style.
    pub fn format_compact(&self) -> String {
        let mut args = String::new();
        for (arg, _) in self.args.val.iter() {
            args += &*format!("({}) ", arg);
        }

        args.pop();

        let mut oargs = String::new();
        for (oarg, (_, sub_arg)) in self.oargs.val.iter() {
            if sub_arg.is_some() {
                let sub_arg_un = sub_arg.as_ref().unwrap();
                if sub_arg_un.1 {
                    oargs += &*format!("|{} [{}]| ", oarg, sub_arg_un.0);
                } else {
                    oargs += &*format!("|{} ({})| ", oarg, sub_arg_un.1);
                }
            } else {
                oargs += &*format!("|{}| ", oarg);
            }
        }

        oargs.pop();

        format!(
            "{} {} {}",
            self.caller,
            args,
            oargs
        )
    }
}


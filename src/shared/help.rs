#![allow(dead_code)]


use super::*;
use std::fmt::Debug;


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


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HelpDisplay {
    Structure,
    Compact,
    // Descriptive,
    Custom,
}

impl Default for HelpDisplay {
    fn default() -> Self {
        Self::Structure
    }
}

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


#[derive(Clone, Default, Debug)]
struct Args {
    val: HashMap<String, String>
}

#[derive(Clone, Default, Debug)]
struct OArgs {
    val: HashMap<String, (String, Option<(String, bool)>)>
}

#[derive(Clone)]
pub struct Help {
    name: String,
    desc: String,
    args: Args,
    oargs: OArgs,
    custom: String,
    display: HelpDisplay,
}

impl Help {
    pub fn new<T: ToString, U: ToString>(name: T, desc: U) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            args: Default::default(),
            oargs: Default::default(),
            custom: String::new(),
            display: Default::default()
        }
    }

    pub fn custom<T: ToString>(help_message: T) -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            args: Default::default(),
            oargs: Default::default(),
            custom: help_message.to_string(),
            display: HelpDisplay::Custom,
        }
    }

    pub fn set_display_structure(mut self) -> Self {
        if self.display == HelpDisplay::Custom {
            return self;
        }

        self.display = HelpDisplay::Structure;
        self
    }

    pub fn set_display_compact(mut self) -> Self {
        if self.display == HelpDisplay::Custom {
            return self;
        }

        self.display = HelpDisplay::Compact;
        self
    }

    pub fn change_desc<T: ToString>(mut self, desc: T) -> Self {
        self.desc = desc.to_string();
        self
    }

    pub fn add_arg<T: ToString, U: ToString>(mut self, name: T, desc: U) -> Self {
        self.args.val.insert(name.to_string(), desc.to_string());
        self
    }

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

    fn display_structure(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.custom.is_empty() && self.display == HelpDisplay::Custom {
            return write!(f, "{}", self.custom);
        }

        match (
            self.args.val.is_empty(),
            self.oargs.val.is_empty()
        ) {
            (true, true) => {
                write!(f, "[{}] - {}", self.name, parse_desc(&self.desc))
            }
            (false, true) => {
                write!(f, "NAME:\n\t[{}] - {}\n\n{}\n", self.name, parse_desc(&self.desc), self.args)
            }
            (true, false) => {
                write!(f, "NAME:\n\t[{}] - {}\n\n{}\n", self.name, parse_desc(&self.desc), self.oargs)
            }
            (false, false) => {
                write!(f, "NAME:\n\t[{}] - {}\n\n{}\n\n{}\n", self.name, parse_desc(&self.desc), self.args, self.oargs)
            }
        }
    }

    fn display_compact(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.custom.is_empty() && self.display == HelpDisplay::Custom {
            return write!(f, "{}", self.custom);
        }

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

        write!(
            f,
            "{} {} {}",
            self.name,
            args,
            oargs
        )
    }

    fn display_custom(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.custom)
    }
}

impl Default for Help {
    fn default() -> Self {
        Self {
            name: String::from("_UNKNOWN"),
            desc: String::from("The Help definition is empty!"),
            args: Default::default(),
            oargs: Default::default(),
            custom: String::new(),
            display: Default::default()
        }
    }
}

// Unresolved conflict
/*impl<C: Command + ?Sized> From<&C> for Help {
    fn from(command: &C) -> Self {
        Self {
            name: command.name().to_string(),
            desc: String::from("help is not implemented for this sync command"),
            args: Default::default(),
            oargs: Default::default()
        }
    }
}

#[cfg(feature = "async")]
impl<A> From<&A> for Help {
    fn from(command: &A) -> Self {
        Self {
            name: command.name().to_string(),
            desc: String::from("help is not implemented for this async command"),
            args: Default::default(),
            oargs: Default::default()
        }
    }
}*/

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

impl Display for Help {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.display {
            HelpDisplay::Structure => (&self).display_structure(f),
            HelpDisplay::Compact => (&self).display_compact(f),
            HelpDisplay::Custom => (&self).display_custom(f),
        }
    }
}

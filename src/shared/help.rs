#![allow(dead_code)]

use super::*;


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
struct HelpData {
    name: String,
    desc: String,
    args: Args,
    oargs: OArgs,
}

enum HelpVariant {
    Standard(HelpData),

    #[deprecated]
    Custom(String),
}

pub struct Help {
    variant: HelpVariant,
}

impl Help {
    pub fn new<T: ToString, U: ToString>(name: T, desc: U) -> Self {
        let h_var = HelpVariant::Standard(
            HelpData {
                name: name.to_string(),
                desc: desc.to_string(),
                args: Default::default(),
                oargs: Default::default(),
            }
        );
        
        Self {
            variant: h_var,
        }
    }

    pub fn custom<T: ToString>(help_message: T) -> Self {
        Self {
            variant: HelpVariant::Custom(help_message.to_string())
        }
    }

    pub fn add_arg<T: ToString, U: ToString>(mut self, name: T, desc: U) -> Self {
        match &mut self.variant {
            HelpVariant::Standard(h_var) => {
                h_var.args.val.insert(name.to_string(), desc.to_string());
            }
            _ => ()
        }

        self
    }

    pub fn add_oarg<T: ToString, U: ToString>(mut self, name: T, desc: U, sub_arg: Option<SubArg>) -> Self {
        match &mut self.variant {
            HelpVariant::Standard(h_var) => {
                let sub_arg_parsed = if sub_arg.is_some() {
                    let sub_arg_unwrapped = sub_arg.unwrap();

                    Some((
                        sub_arg_unwrapped.name,
                        sub_arg_unwrapped.is_optional,
                    ))
                } else {
                    None
                };

                h_var.oargs.val.insert(
                    name.to_string(),
                    (desc.to_string(), sub_arg_parsed)
                );
            }
            _ => ()
        }

        self
    }

    pub fn format_custom(&self) -> String {
        return match &self.variant {
            HelpVariant::Standard(_) => self.format_compact(),
            HelpVariant::Custom(custom) => format!("{}", custom),
        }

    }

    pub fn format_structure(&self) -> String {
        return match &self.variant {
            HelpVariant::Standard(h_var) => {
                match (
                    h_var.args.val.is_empty(),
                    h_var.oargs.val.is_empty()
                ) {
                    (true, true) => {
                        format!("[{}] - {}", h_var.name, parse_desc(&h_var.desc))
                    }
                    (false, true) => {
                        format!("NAME:\n\t[{}] - {}\n\n{}\n", h_var.name, parse_desc(&h_var.desc), h_var.args)
                    }
                    (true, false) => {
                        format!("NAME:\n\t[{}] - {}\n\n{}\n", h_var.name, parse_desc(&h_var.desc), h_var.oargs)
                    }
                    (false, false) => {
                        format!("NAME:\n\t[{}] - {}\n\n{}\n\n{}\n", h_var.name, parse_desc(&h_var.desc), h_var.args, h_var.oargs)
                    }
                }
            }

            HelpVariant::Custom(_) => {
                self.format_custom()
            }
        }
    }

    pub fn format_compact(&self) -> String {
        return match &self.variant {
            HelpVariant::Standard(h_var) => {
                let mut args = String::new();
                for (arg, _) in h_var.args.val.iter() {
                    args += &*format!("({}) ", arg);
                }

                args.pop();

                let mut oargs = String::new();
                for (oarg, (_, sub_arg)) in h_var.oargs.val.iter() {
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
                    h_var.name,
                    args,
                    oargs
                )
            }

            HelpVariant::Custom(_) => {
                self.format_custom()
            }
        }
    }

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

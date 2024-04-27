use std::collections::HashMap;
use crate::Error;

const FLAG_PREFIX: &str = "--";

/// Arguments format structure used to deserialize raw inputs.
///
/// The format of instruction is as follows:
/// ```pseudo
/// <caller> <arg> --<o_arg> <sub_arg>
/// ```
///
/// Where:
/// - `caller` - Command caller, used to determine which command to use.
/// - `arg` - Positional argument where the position matters.
/// - `o_arg` - Optional argument. These arguments can be placed in any order.
/// - `sub_arg` - Sub argument that is a child of `o_arg`.
///
/// These can also be chained:
/// ```pseudo
/// <caller> <arg> <arg> --<o_arg> <sub_arg> <sub_arg> --<o_arg>
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Instruction<'a> {
    pub caller: &'a str,
    pub args: Vec<&'a str>,
    pub o_args: HashMap<&'a str, Option<Vec<&'a str>>>,
    pub input: &'a str,
}

impl<'a> Instruction<'a> {
    pub fn new<T: AsRef<str> + ?Sized>(input: &'a T) -> Result<Instruction<'a>, Error> {
        let mut instruction = Self::empty();
        instruction.input = input.as_ref();

        let input = instruction.input;

        let mut split = input
            .split(' ')
            .map(str::trim)
            .filter(|part| !part.is_empty());

        if let Some(part) = split.next() {
            instruction.caller = part;
        } else {
            return Err(Error::InstructionMissingCaller);
        }

        let mut current_o_arg = "";
        let mut is_pos_args = true;
        for part in split {
            if is_pos_args {
                if part.starts_with(FLAG_PREFIX) {
                    is_pos_args = false;
                } else {
                    instruction.args.push(part);
                }
            }

            if !is_pos_args {
                if part.starts_with(FLAG_PREFIX) {
                    instruction.o_args.insert(part, None);
                    current_o_arg = part;
                } else {
                    let sub_args = instruction
                        .o_args
                        .get_mut(current_o_arg)
                        .ok_or_else(|| Error::InstructionSubArgWithoutOArg)?;

                    if let Some(sub_args) = sub_args {
                        sub_args.push(part);
                    } else {
                        *sub_args = Some(vec![part]);
                    }
                }
            }
        }

        Ok(instruction)
    }

    fn empty() -> Instruction<'a> {
        Self {
            caller: "",
            args: Vec::new(),
            o_args: HashMap::new(),
            input: "",
        }
    }
}

#[cfg(test)]
impl Default for Instruction<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

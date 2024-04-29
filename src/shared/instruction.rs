use std::collections::HashMap;
use crate::Error;

const FLAG_PREFIX: &str = "--";
const SYMBOL_SPACE: char = ' ';
const SYMBOL_QUOTES: char = '"';
const SYMBOL_HASH: char = '#';

#[derive(Default, Debug)]
struct State<'a> {
    buffer: Vec<&'a str>,
    start: Option<usize>,
    end: Option<usize>,
    ignore_space: bool,
    collecting: bool,
    previous: Option<char>,
}

impl<'a> State<'a> {
    fn push_part(&mut self, pos: usize, input: &'a str) {
        if self.start.is_none() {
            return;
        }

        let start = self.start.take().unwrap();
        let end = *self.end.insert(pos);
        let part = &input[start..end];

        self.buffer.push(part);
    }
}

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

        let mut state = State::default();
        for (pos, char) in input.chars().enumerate() {
            if state.collecting || state.ignore_space || char != SYMBOL_SPACE {
                if !state.collecting && char == SYMBOL_QUOTES {
                    if state.previous == Some(SYMBOL_HASH) {
                        state.collecting = true;
                    } else {
                        if state.ignore_space {
                            state.push_part(pos, input);
                            state.ignore_space = false
                        } else {
                            state.ignore_space = true;
                        }
                    }
                } else {
                    if state.collecting && char == SYMBOL_HASH && state.previous == Some(SYMBOL_QUOTES) {
                        state.collecting = false;
                        state.start = state.start.map(|pos| pos+2);
                        state.push_part(pos-1, input);
                    } else {
                        if state.start.is_none() {
                            state.start = Some(pos);
                        } else {
                            let _ = state.end.insert(pos);
                        }
                    }
                }
            } else {
                state.push_part(pos, input);
            }

            state.previous = Some(char);
        }

        if let Some(start) = state.start {
            if let Some(end) = state.end {
                let part = &input[start..=end];
                if !part.is_empty() {
                    state.buffer.push(part);
                }
            }
        }

        let mut split = state.buffer.into_iter();

        if let Some(part) = split.next() {
            instruction.caller = part;
        } else {
            return Err(Error::InstructionMissingCaller);
        }

        let mut current_o_arg = "#";
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

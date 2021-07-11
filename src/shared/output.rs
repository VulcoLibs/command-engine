use super::*;

pub const RESULT_MAX_PRIME: u16 = 4095;
const RESULT_DEF_OK: u16 = 0xA000;
const RESULT_DEF_ERROR: u16 = 0xF000;


/// Enum that is a part of an Output structure.
///
/// **Ok** and **Error** contain integer representing some status code.
/// Each should be documented in the command help.
///
/// Restricted prime values:
/// * `Error(0)` - Engine failure
/// * `Ok(0)` - Help has been called
///
/// Restricted values shouldn't be used in a custom commands,
/// but returning an Output with restricted prime value won't panic
///
/// If everything was completed successfully without any info,
/// **Ok** should contain prime value `1`, so
/// the status code would be `0xA001`.
///
/// If something has failed without any info,
/// **Error** should contain prime value `1`, so
/// the status code would be `0xF001`.
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Result {
    Ok(u16),
    Error(u16),
}

impl Result {
    fn parse_status_code(res: &str, def_val: u16, val: u16) -> u16 {
        if val > RESULT_MAX_PRIME {
            panic!("Exceed max value of {} status code!\nMax value is: [{}]\nProvided value was: [{}]", res, RESULT_MAX_PRIME, val)
        }

        def_val + val
    }

    /// Indicate Successful result status code by a prime value
    ///
    /// Panics when `prime_val` is over `4095`
    pub fn ok(prime_val: u16) -> Self {
        Self::Ok(Self::parse_status_code(
            "Ok",
            RESULT_DEF_OK,
            prime_val,
        ))
    }

    /// Indicate Failed result status code by a prime value
    ///
    /// Panics when `prime_val` is over `4095`
    pub fn err(prime_val: u16) -> Self {
        Self::Error(Self::parse_status_code(
            "Error",
            RESULT_DEF_ERROR,
            prime_val,
        ))
    }

    /// Return the full status code of the Result.
    ///
    /// (prefix + prime value)
    pub fn status_code(&self) -> u16 {
        return match self {
            Result::Ok(val) => *val,
            Result::Error(val) => *val,
        }
    }

    /// Return only prime value of the Result
    pub fn raw_val(&self) -> u16 {
        return match self {
            Result::Ok(val) => *val - RESULT_DEF_OK,
            Result::Error(val) => *val - RESULT_DEF_ERROR,
        }
    }

    pub fn is_ok(&self) -> bool {
        match &self {
            &Result::Ok(_) => true,
            &Result::Error(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        !(&self).is_ok()
    }
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "0x{:X}",
            self.status_code(),
        )
    }
}


/// Wrapper for the Command's output
pub struct Output {
    pub result: Result,
    pub message: String,
}

impl Output {
    /// Creates new Output object
    ///
    /// # Arguments
    ///
    /// * `result` - Enum defining if the output was successful or not
    /// * `msg` - Message, can be None
    fn new<S: ToString>(result: Result, msg: Option<S>) -> Self{
        let message = match msg {
            None => "".to_string(),
            Some(message) => message.to_string(),
        };

        Self {
            result,
            message,
        }
    }

    /// Returns a successful Output with a certain value and message
    ///
    /// # Arguments
    ///
    /// * `prime_val` - Value output identifier. Can't be over 4095 or it will panic
    /// * `msg` - Message, can be None
    pub fn new_ok<S: ToString>(prime_val: u16, msg: Option<S>) -> Self {
        Self::new(
            Result::ok(prime_val),
            msg,
        )
    }

    /// Returns a failed Output with a certain value and message
    ///
    /// # Arguments
    ///
    /// * `prime_val` - Value output identifier. Can't be over 4095 or it will panic
    /// * `msg` - Message, can be None
    pub fn new_error<S: ToString>(prime_val: u16, msg: Option<S>) -> Self {
        Self::new(
            Result::err(prime_val),
            msg,
        )
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} - [{}]",
            self.result,
            self.message,
        )
    }
}

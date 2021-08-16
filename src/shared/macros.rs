/// This macro unwraps the Ok and returns Err from Result.
///
/// Example:
/// ```rust
/// fn get_ok_result() -> Result<u16, &'static str> {
///     Ok(5_u16)
/// }
///
/// fn get_err_result() -> Result<u16, &'static str> {
///     Err("error")
/// }
///
/// fn test() -> &'static str {
///     let num = residual!(get_ok_result());
///     println!("u16 from get_ok_result: {}", num);
///
///     let _ = residual!(get_err_result());
///     "This wont be returned, because \"error\" string has been returned by residual!(get_err_result());"
/// }
/// ```
macro_rules! residual {
    ($f:expr) => {
        match $f {
            Ok(result) => result,
            Err(error) => return error,
        }
    };
}

pub (crate) use residual;

use std::fmt;

/// A custom error type for the library.
#[derive(Debug)]
pub struct CustomError {
    details: String,
}

impl CustomError {
    /// Creates a new custom error with the given message.
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message.
    ///
    /// # Returns
    ///
    /// A new `CustomError`.
    pub fn new(msg: &str) -> CustomError {
        CustomError { details: msg.to_string() }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}

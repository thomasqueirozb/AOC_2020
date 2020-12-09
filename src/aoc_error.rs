use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AOCError {
    details: String,
}

impl AOCError {
    pub fn new<S: Into<String>>(msg: S) -> AOCError {
        AOCError {
            details: msg.into(),
        }
    }
}

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AOCError {
    fn description(&self) -> &str {
        &self.details
    }
}

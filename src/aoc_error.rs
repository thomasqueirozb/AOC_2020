use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AOCError {
    details: String,
}

impl AOCError {
    pub fn new<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        AOCError {
            details: msg.into(),
        }
    }
}

impl From<&str> for AOCError {
    fn from(msg: &str) -> Self {
        Self::new(msg)
    }
}

impl From<&String> for AOCError {
    fn from(msg: &String) -> Self {
        Self::new(msg)
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

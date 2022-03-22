use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SimpleError {
    message: String,
}
impl SimpleError {
    pub fn new( message: &str) -> Self {
        Self {
            message: String::from_str(message).unwrap()
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}
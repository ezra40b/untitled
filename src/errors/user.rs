use std::{error::Error, fmt};

#[derive(Debug)]
pub struct UserError {
    msg: &'static str
}

impl Error for UserError {}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl UserError {
    pub fn new(msg: &'static str) -> UserError {
        UserError { msg }
    }
}
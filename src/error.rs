use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    MissingIndex { member: String, index: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MissingIndex {ref member, ref index} => write!(f, "MissingIndex error: {} not in {}", member, index),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MissingIndex { .. } => "Missing Index",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}

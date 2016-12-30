use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    NoPixbuf,
}


impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IOError(_) => "Could not open file",
            Error::NoPixbuf => "Pixbuf was null",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IOError(ref e) => Some(e),
            Error::NoPixbuf => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

use glib;
use std::error::Error as StdError;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    PathUTF8Error,
    FileNotFound(glib::Error),
    GLibError(glib::Error),
}


impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PathUTF8Error => "Error while parsing path as utf-8",
            Error::FileNotFound(_) => "Could not open file",
            Error::GLibError(_) => "A GLib error happened",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::PathUTF8Error => None,
            Error::FileNotFound(ref e) |
            Error::GLibError(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

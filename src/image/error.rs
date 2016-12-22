use std::error;
use std::fmt;
use glib;

#[derive(Debug)]
pub enum Error {
    FileNotFound(glib::FileError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}

impl error::Error for Error {
    fn description(&self) -> &str {

    }
}

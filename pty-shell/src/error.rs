use std::{self, fmt, io};

use tty;

#[derive(Debug)]
pub enum Error {
    Pty(tty::ForkError),
    Io(io::Error),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "pty-shell error"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Pty(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<tty::ForkError> for Error {
    fn from(err: tty::ForkError) -> Error {
        Error::Pty(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

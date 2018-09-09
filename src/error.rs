use std::{io, result, error};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    PbIo(io::Error),
    PbOther(Box<error::Error>),
}

impl Error {
    #[inline]
    pub(crate) fn unexpected_eof() -> Error {
        Error::PbIo(io::Error::new(io::ErrorKind::UnexpectedEof, ""))
    }

    #[inline]
    pub(crate) fn invalid_data() -> Error {
        Error::PbIo(io::Error::new(io::ErrorKind::InvalidData, ""))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::PbIo(e)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::PbIo(ref e) => write!(formatter, "PbIo({})", e),
            Error::PbOther(ref o) => write!(formatter, "PbOther({})", o),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PbIo(_) => "PbIo",
            Error::PbOther(_) => "PbOther",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::PbIo(ref e) => Some(e),
            Error::PbOther(ref o) => Some(o.as_ref()),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

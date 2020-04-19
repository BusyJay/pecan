use pecan_utils::codec;
use std::fmt::{self, Display, Formatter};
use std::{error, result};

#[derive(Debug)]
pub enum Error {
    ExceedRecursiveLimit(usize),
    InvalidData {
        wire: u8,
        index: u8,
        reason: codec::Error,
    },
    OutOfSpace,
}

impl Error {
    pub(crate) fn truncated() -> Error {
        Error::InvalidData {
            wire: 0,
            index: 0,
            reason: codec::Error::Truncated,
        }
    }

    pub(crate) fn corrupted() -> Error {
        Error::InvalidData {
            wire: 0,
            index: 0,
            reason: codec::Error::Corrupted,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::ExceedRecursiveLimit(limit) => write!(f, "recursive too deep > {}", limit),
            Error::InvalidData {
                wire,
                index,
                reason,
            } => write!(
                f,
                "invalid data for [wire: {}, index: {}]: {:?}",
                wire, index, reason
            ),
            Error::OutOfSpace => write!(f, "out of space"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ExceedRecursiveLimit(_) => "ExceedRecursiveLimit",
            Error::InvalidData { .. } => "InvalidData",
            Error::OutOfSpace => "OutOfSpace",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub type Result<T> = result::Result<T, Error>;

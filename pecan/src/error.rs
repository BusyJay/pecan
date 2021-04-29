#[derive(Debug, PartialEq)]
pub enum Error {
    WantMore(u64, u8),
    Corrupted,
    Eof,
    DepthExcceedLimit(u32),
    SizeExcceedLimit(u64),
}

impl Error {
    pub fn corrupted() -> Error {
        // panic!("corrupted");
        Error::Corrupted
    }

    pub fn eof() -> Error {
        // panic!("eof");
        Error::Eof
    }
}

pub type Result<T> = std::result::Result<T, Error>;

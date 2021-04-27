#[derive(Debug, PartialEq)]
pub enum Error {
    WantMore(u64, u8),
    Corrupted,
    Eof,
    DepthExcceedLimit(u32),
}

impl Error {
    pub fn corrupted() -> Error {
        panic!("corrupted");
    }

    pub fn eof() -> Error {
        panic!("eof");
    }
}

pub type Result<T> = std::result::Result<T, Error>;

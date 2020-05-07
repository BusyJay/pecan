#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::len_without_is_empty)]

mod enumerate;
mod error;
mod message;
mod stream;

pub use enumerate::EnumType;
pub use error::{Error, Result};
pub use message::{CacheSize, Message};
pub use stream::{encoded, Buf, BufMut, CodedInputStream, CodedOutputStream};
pub use pecan_utils::{codec, naming};

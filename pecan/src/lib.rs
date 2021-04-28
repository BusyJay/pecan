mod codec;
mod error;
mod extension;
mod message;
pub mod reflection;
pub mod stream;

pub use self::error::{Error, Result};
pub use self::extension::{Extension, ExtensionMap};
pub use self::message::{DefaultInstance, Enumerate, Message};
pub use self::stream::{CodedInputStream, CodedOutputStream};
pub use bytes::{Buf, BufMut, Bytes};
pub use lazy_static::lazy_static;
pub use std::collections::HashMap;

pub mod prelude {
    pub use crate::message::{DefaultInstance, Enumerate, Message};
    pub use crate::stream::{CodedInputStream, CodedOutputStream};

    pub use crate::stream::field::*;
}

use std::collections::HashMap;

use bytes::Bytes;

pub struct Extension<T> {
    number: u32,
}

pub struct ExtensionMap {
    // Use an `Option` to make it possible to initialize in const context.
    map: Option<HashMap<u32, Bytes>>,
}

impl ExtensionMap {
    pub const fn new() -> ExtensionMap {
        ExtensionMap {
            map: None,
        }
    }

    pub fn get(&self, extension: &Extension<T>) -> T {

    }
}

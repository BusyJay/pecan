use std::marker::PhantomData;
use std::{
    collections::{hash_map::Values, HashMap},
    fmt::{self, Debug, Formatter},
};

use crate::prelude::*;
use crate::Result;
use bytes::{Bytes, BytesMut};

pub struct Extension<T, C> {
    tag: u64,
    _marker: PhantomData<(T, C)>,
}

impl<T, C> Extension<T, C> {
    pub const fn new(tag: u64) -> Extension<T, C> {
        Extension {
            tag,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct ExtensionMap {
    // Use an `Option` to make it possible to initialize in const context.
    map: Option<HashMap<u64, Bytes>>,
}

impl ExtensionMap {
    pub const fn new() -> ExtensionMap {
        ExtensionMap { map: None }
    }

    pub fn get<T, C>(&self, extension: &Extension<T, C>) -> Result<Option<T>>
    where
        C: ReadFieldCodec<T> + MergeFieldCodec<T>,
    {
        let map = match &self.map {
            Some(m) => m,
            None => return Ok(None),
        };
        let mut buf = match map.get(&extension.tag) {
            Some(b) => b.clone(),
            None => return Ok(None),
        };
        let mut input = CodedInputStream::new(&mut buf);
        let mut v = C::read_from(&mut input)?;
        if !input.is_empty() {
            C::merge_from(&mut v, &mut input)?;
        }
        Ok(Some(v))
    }

    pub fn insert_copy<T, C>(&mut self, extension: &Extension<T, C>, t: T) -> Result<()>
    where
        T: Copy,
        C: WriteFieldCodec<T>,
    {
        let mut bytes = BytesMut::with_capacity(C::len(t) as usize);
        let mut output = CodedOutputStream::new(&mut bytes);
        C::write_to(t, &mut output)?;
        drop(output);
        self.insert_raw(extension.tag, bytes.freeze());
        Ok(())
    }

    pub fn insert_ref<'a, T, C>(&'a mut self, extension: &Extension<T, C>, t: &'a T) -> Result<()>
    where
        T: Copy,
        C: WriteFieldCodec<&'a T>,
    {
        let mut bytes = BytesMut::with_capacity(C::len(t) as usize);
        let mut output = CodedOutputStream::new(&mut bytes);
        C::write_to(t, &mut output)?;
        drop(output);
        self.insert_raw(extension.tag, bytes.freeze());
        Ok(())
    }

    pub(crate) fn insert_raw(&mut self, tag: u64, bytes: Bytes) {
        let m = self.map.get_or_insert_with(Default::default);
        m.insert(tag, bytes);
    }

    pub(crate) fn get_raw(&self, tag: u64) -> Option<&Bytes> {
        self.map.as_ref().and_then(|m| m.get(&tag))
    }

    pub(crate) fn values_raw(&self) -> Option<Values<u64, Bytes>> {
        self.map.as_ref().map(|m| m.values())
    }

    pub fn len(&self) -> u64 {
        self.values_raw()
            .map_or(0, |v| v.map(|b| b.len() as u64).sum())
    }

    pub fn is_empty(&self) -> bool {
        self.map.as_ref().map_or(true, |m| m.is_empty())
    }
}

impl Debug for ExtensionMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_list();
        if let Some(m) = &self.map {
            f.entries(m.keys());
        }
        f.finish()
    }
}

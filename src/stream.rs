use std::mem::MaybeUninit;
use std::{slice, usize};

mod input;
mod output;

pub use self::input::CodedInputStream;
pub use self::output::{encoded, CodedOutputStream};

pub trait Buf {
    fn available(&self) -> usize;
    fn bytes(&self) -> &[u8];
    fn advance(&mut self, cnt: usize);
}

pub trait BufMut {
    /// # Safety
    ///
    /// Should ensure data are valid before calling the function.
    unsafe fn advance_mut(&mut self, cnt: usize);
    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>];
    fn reserve(&mut self, addition: usize);
}

impl Buf for &'_ [u8] {
    #[inline]
    fn available(&self) -> usize {
        self.len()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        *self = &self[cnt..]
    }
}

impl BufMut for Vec<u8> {
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let new_len = self.len() + cnt;
        if new_len <= self.capacity() {
            self.set_len(new_len);
            return;
        }
        Vec::reserve(self, cnt);
        self.set_len(new_len);
    }

    #[inline]
    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        if self.capacity() > self.len() {
            unsafe {
                let ptr = self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<u8>;
                return slice::from_raw_parts_mut(ptr, self.capacity() - self.len());
            }
        }
        self.reserve(8);
        unsafe {
            let ptr = self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<u8>;
            slice::from_raw_parts_mut(ptr, self.capacity() - self.len())
        }
    }

    #[inline]
    fn reserve(&mut self, addition: usize) {
        Vec::reserve(self, addition)
    }
}

impl BufMut for &'_ mut Vec<u8> {
    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        BufMut::advance_mut(*self, cnt)
    }

    #[inline]
    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        BufMut::bytes_mut(*self)
    }

    #[inline]
    fn reserve(&mut self, addition: usize) {
        BufMut::reserve(*self, addition)
    }
}

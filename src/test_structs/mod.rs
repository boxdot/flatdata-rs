#![cfg(test)]

use std::convert;
use std::mem;
use std::slice;

use archive::{Index, IndexMut, Struct, StructMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Idx {
    data: *const u8,
}

impl Struct for Idx {
    const SCHEMA: &'static str = "Index";
    const SIZE_IN_BYTES: usize = 4;
    type Mut = IdxMut;
    fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl convert::From<*const u8> for Idx {
    fn from(data: *const u8) -> Self {
        Self { data }
    }
}

impl Index for Idx {
    type IndexMut = IdxMut;
    fn value(&self) -> usize {
        read_bytes!(u64, self.data, 0, 32) as usize
    }
}

#[derive(Debug)]
pub struct IdxMut {
    data: *mut u8,
}

impl convert::From<*mut u8> for IdxMut {
    fn from(data: *mut u8) -> Self {
        Self { data }
    }
}

impl StructMut for IdxMut {
    type Const = Idx;
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data
    }
}

impl AsRef<Idx> for IdxMut {
    fn as_ref(&self) -> &Idx {
        unsafe { mem::transmute(self) }
    }
}

impl IndexMut for IdxMut {
    fn set_value(&mut self, value: usize) {
        let buffer = unsafe { slice::from_raw_parts_mut(self.data, Self::Const::SIZE_IN_BYTES) };
        write_bytes!(u32; value as u32, buffer, 0, 32);
    }
}

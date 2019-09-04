use std::io::{Read, Seek};

use crate::error::Result;
use crate::isa;

pub struct ReadSeekInput<T> {
    inner: T
}

impl<T> ReadSeekInput<T>
where 
    T: Read + Seek
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> ReadSeekInput<T> 
where 
    T: Read + Seek 
{
    pub fn next(&mut self) -> Result<isa::InstructionOwned> {
        unimplemented!()
    }
}

pub struct SliceInput<'a> {
    slice: &'a [u8],
    pc: usize
}

impl<'a> SliceInput<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, pc: 0 }
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.slice
    }
}

impl<'a> SliceInput<'a> {
    pub fn next(&mut self) -> Result<&isa::Instruction> {
        let first_byte = self.slice[self.pc];
        let length_bytes = if first_byte & 0b11 != 0b11 {
            2
        } else if first_byte & 0b00111 != 0b00111 {
            4
        } else if first_byte & 0b000001 != 0b000001 {
            6
        } else if first_byte & 0b0000001 != 0b0000001 {
            8
        } else {
            unimplemented!()
        };
        let range = self.pc..(self.pc + length_bytes);
        self.pc += length_bytes;
        Ok(isa::Instruction::new(&self.slice[range]))
    }
}

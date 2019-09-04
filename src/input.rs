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
        let mut buf = isa::InstructionOwned::new();
        let mut tmp = [0u8; 1];
        self.inner.read(&mut tmp)?;
        let length_bytes = length_from_first_byte(tmp[0]);
        buf.push(tmp[0]);
        for _ in 1..length_bytes {
            self.inner.read(&mut tmp)?;
            buf.push(tmp[0]);
        }
        Ok(buf)
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
        let length_bytes = length_from_first_byte(first_byte);
        let range = self.pc..(self.pc + length_bytes);
        self.pc += length_bytes;
        Ok(isa::Instruction::new(&self.slice[range]))
    }
}

#[inline]
fn length_from_first_byte(first_byte: u8) -> usize {
    if first_byte & 0b11 != 0b11 {
        2
    } else if first_byte & 0b00111 != 0b00111 {
        4
    } else if first_byte & 0b000001 != 0b000001 {
        6
    } else if first_byte & 0b0000001 != 0b0000001 {
        8
    } else {
        unimplemented!()
    }
} 

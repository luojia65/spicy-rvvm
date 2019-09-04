use std::io::{Read, Seek, SeekFrom};

use crate::error::Result;
use crate::isa;

pub struct ReadSeekInput<T> {
    inner: T,
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
        let mut tmp = [0u8; 2];
        self.inner.read(&mut tmp)?;
        let length_bytes = length_from_first_byte(tmp[0], tmp[1]);
        buf.push(tmp[0]);
        buf.push(tmp[1]);
        for _ in 1..(length_bytes / 2) {
            self.inner.read(&mut tmp)?;
            buf.push(tmp[0]);
            buf.push(tmp[1]);
        }
        Ok(buf)
    }

    pub fn set_pc(&mut self, new_pc: usize) -> Result<()> {
        self.inner.seek(SeekFrom::Start(new_pc as u64))?;
        Ok(())
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
        let second_byte = self.slice[self.pc + 1];
        let length_bytes = length_from_first_byte(first_byte, second_byte);
        let range = self.pc..(self.pc + length_bytes);
        self.pc += length_bytes;
        Ok(isa::Instruction::new(&self.slice[range]))
    }

    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }
}

#[inline]
fn length_from_first_byte(first_byte: u8, second_byte: u8) -> usize {
    if first_byte & 0b11 != 0b11 {
        2
    } else if first_byte & 0b00111 != 0b00111 {
        4
    } else if first_byte & 0b000001 != 0b000001 {
        6
    } else if first_byte & 0b0000001 != 0b0000001 {
        8
    } else {
        let bits = (second_byte & 0b01110000) >> 4;
        if bits != 0b0111 {
            (2 * bits + 10) as usize
        } else {
            unimplemented!()
        }
    }
} 

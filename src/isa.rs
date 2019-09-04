pub mod i;
pub mod c;

use std::borrow::Borrow;

#[derive(Debug)]
pub struct Instruction {
    text_slice: [u8]
}

pub struct InstructionOwned {
    text_vec: Vec<u8>
}

impl Instruction {
    pub fn new<T: AsRef<[u8]> + ?Sized>(src: &T) -> &Self {
        unsafe { &*(src.as_ref() as *const [u8] as *const Instruction) }
    }
}

impl InstructionOwned {
    pub fn new() -> Self {
        InstructionOwned { text_vec: Vec::new() }
    }

    pub fn push(&mut self, byte: u8) {
        self.text_vec.push(byte)
    }
}

impl ToOwned for Instruction {
    type Owned = InstructionOwned;

    fn to_owned(&self) -> Self::Owned {
        InstructionOwned { text_vec: self.text_slice.to_owned() }
    }
}

impl Borrow<Instruction> for InstructionOwned {
    fn borrow(&self) -> &Instruction {
        Instruction::new(&self.text_vec)
    }
}

impl Instruction {
    pub fn execute(&self) {
        unimplemented!()
    }
}

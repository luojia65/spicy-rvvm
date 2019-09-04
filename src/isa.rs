pub mod i;

use std::borrow::Borrow;

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

impl Instruction {
    pub fn execute(&self) {
        unimplemented!()
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
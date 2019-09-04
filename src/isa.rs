pub mod i;
pub mod c;

use std::borrow::Borrow;
use crate::runtime;

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

pub enum Type {
    R { opcode: u8, rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8 },
    I { opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: i16 },
    S { opcode: u8, rs1: u8, rs2: u8, funct3: u8, imm: i16 },
    B { opcode: u8, rs1: u8, rs2: u8, funct3: u8, imm: i16 },
    U { opcode: u8, rd: u8, imm: i32 },
    J { opcode: u8, rd: u8, imm: i32 },
}

const OPCODE_JALR: u8 =     0b111_0011; 

impl Type {
    pub fn decode(ins: &Instruction) -> Self {
        let opcode = ins.text_slice[0] & 0x7F;
        match opcode {
            OPCODE_JALR => {
                let i = u32::from_le_bytes([
                    ins.text_slice[0], ins.text_slice[1], 
                    ins.text_slice[2], ins.text_slice[3]
                ]);
                let rd = (i >> 7) as u8 & 0b1_1111;
                let funct3 = (i >> 12) as u8 & 0b111;
                let rs1 = (i >> 15) as u8 & 0b1_1111;
                let mut imm = (i >> 20) as i16 & 0b1111_1111_1111;
                if imm & 0b1000_0000_0000 != 0 {
                    imm = -imm;
                }
                Type::I { opcode, rd, funct3, rs1, imm }
            },
            _ => unimplemented!()
        }
    }

    #[inline]
    pub fn opcode(&self) -> u8 {
        match *self {
            Type::R { opcode, .. } => opcode,
            Type::I { opcode, .. } => opcode,
            Type::S { opcode, .. } => opcode,
            Type::B { opcode, .. } => opcode,
            Type::U { opcode, .. } => opcode,
            Type::J { opcode, .. } => opcode,
        }
    }

    pub fn execute_on(&self, rt: &mut runtime::Runtime) {
        match *self {
            Type::I { opcode: _, rd, funct3: _, rs1, imm } => 
                i::jalr(rt, rd, rs1, imm),
            _ => unimplemented!()
        }
    }
}

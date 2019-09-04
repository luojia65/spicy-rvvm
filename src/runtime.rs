use crate::isa;

#[derive(Debug)]
pub struct Runtime {
    int_reg: [u64; 32],
    pc: u64,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            int_reg: [0; 32],
            pc: 0
        }
    }
}

impl Runtime {
    #[inline]
    pub fn set_pc(&mut self, new_pc: u64) {
        self.pc = new_pc;
    }

    #[inline]
    pub fn pc(&self) -> u64 {
        self.pc
    }

    #[inline]
    pub fn set_int(&mut self, index: u8, value: u64) {
        if index != 0 {
            self.int_reg[index as usize] = value;
        }
    }

    #[inline]
    pub fn int(&self, index: u8) -> u64 {
        self.int_reg[index as usize]
    }
}

impl Runtime {
    #[inline]
    pub fn execute(&mut self, ins: &isa::Instruction) {
        let ty = isa::Type::decode(ins);
        ty.execute_on(self)
    }
}

use crate::isa;

#[derive(Debug)]
pub struct Core {
    int_reg: [u64; 32],
    pc: u64,
}

impl Core {
    #[inline]
    pub fn new() -> Self {
        Self::with_initial_pc(0)
    }

    #[inline]
    pub fn with_initial_pc(pc: u64) -> Self {
        Self { int_reg: [0; 32], pc }
    }
}

impl Core {
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

impl Core {
    #[inline]
    pub fn execute(&mut self, ins: &isa::Instruction) {
        let ty = isa::Type::decode(ins);
        ty.execute_on(self)
    }
}

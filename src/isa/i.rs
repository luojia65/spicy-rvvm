use crate::core::Core;

#[inline]
pub fn jalr(core: &mut Core, rd: u8, rs1: u8, imm: i16) {
    core.set_int(rd, core.pc() + 4);
    core.set_pc((core.int(rs1) as i128 + imm as i128) as u64);
}


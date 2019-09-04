use crate::runtime::Runtime;

#[inline]
pub fn jalr(rt: &mut Runtime, rd: u8, rs1: u8, imm: i16) {
    rt.set_int(rd, rt.pc() + 4);
    rt.set_pc((rt.int(rs1) as i128 + imm as i128) as u64);
}


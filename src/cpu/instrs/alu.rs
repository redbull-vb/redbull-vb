use crate::cpu::Cpu;

impl Cpu {
    // reg2 = reg2 + (sign extend) imm
    // Cycles: 1
    // Flags affected: Zero, Sign, Carry, Overflow
    // Opcode: 0b010001
    pub fn addi_short (&mut self, instr: u16) {
        let reg2_idx = (instr >> 5 & 0x1F) as usize;
        let reg2 = self.regs.gprs[reg2_idx];
        let imm = ((instr as i32) << 27 >> 27) as u32;

        let (res, overflow) = (reg2 as i32).overflowing_add(imm as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(reg2 > !imm); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);

        self.regs.gprs[reg2_idx] = res;
    }
}

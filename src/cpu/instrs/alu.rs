use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    // reg2 = reg2 + (sign extend) imm
    // Cycles: 1
    // Flags affected: Zero, Sign, Carry, Overflow
    // Opcode: 0b010001
    pub fn addi_short (&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg2 = self.regs.gprs[reg2_index];
        let imm = ((instr as i32) << 27 >> 27) as u32; // sign extend immediate

        let (res, overflow) = (reg2 as i32).overflowing_add(imm as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(res < reg2); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);

        self.regs.gprs[reg2_index] = res;
    }

    // (discard) reg2 - (sign extend) imm
    // Cycles: 1
    // Flags affected: Zero, Sign, Carry, Overflow
    // Opcode: 0b010011
    pub fn cmp_imm (&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg2 = self.regs.gprs[reg2_index];
        let imm = ((instr as i32) << 27 >> 27) as u32; // sign extend immediate

        let (res, overflow) = (reg2 as i32).overflowing_sub(imm as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(imm > reg2); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);
    }

    // (discard) reg2 - reg1
    // Cycles: 1
    // Flags affected: Zero, Sign, Carry, Overflow
    // Opcode: 0b000011
    pub fn cmp_reg (&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg2 = self.regs.gprs[reg2_index];
        let reg1 = self.regs.gprs[instr as usize & 0x1F];

        let (res, overflow) = (reg2 as i32).overflowing_sub(reg1 as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(reg1 > reg2); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);
    }
}

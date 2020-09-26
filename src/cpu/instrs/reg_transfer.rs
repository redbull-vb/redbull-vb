use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    // reg2 = reg1 + (imm << 16)
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movhi(&mut self, instr: u16, bus: &Bus) {
        let reg1_idx = instr as usize & 0x1F;
        let reg2_idx = (instr as usize >> 5) & 0x1F;

        self.regs.gprs[reg2_idx] =
            self.regs.gprs[reg1_idx].wrapping_add((self.consume_halfword(bus) as u32) << 16);
    }

    // reg2 = reg1 + (sign extend) imm
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movea(&mut self, instr: u16, bus: &Bus) {
        let reg1_idx = instr as usize & 0x1F;
        let reg2_idx = (instr as usize >> 5) & 0x1F;

        self.regs.gprs[reg2_idx] =
            self.regs.gprs[reg1_idx].wrapping_add(self.consume_halfword(bus) as i16 as u32);
    }
}

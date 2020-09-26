use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    // reg2 = reg1 + (imm << 16)
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movhi(&mut self, bus: &mut Bus, instr: u16) {
        let reg1_index = instr as usize & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let offset = (self.consume_halfword(bus) as u32) << 16;

        self.regs.gprs[reg2_index] = self.regs.gprs[reg1_index].wrapping_add(offset);
    }

    // reg2 = reg1 + (sign extend) imm
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movea(&mut self, bus: &mut Bus, instr: u16) {
        let reg1_index = instr as usize & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let offset = self.consume_halfword(bus) as i16 as u32; // Fetch immediate and sign extend ti

        self.regs.gprs[reg2_index] = self.regs.gprs[reg1_index].wrapping_add(offset);
    }
}

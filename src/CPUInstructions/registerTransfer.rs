use crate::cpu::CPU;
use crate::bus::Bus;
use crate::helpers::signExtendHalfword;

impl CPU {
    // reg2 = reg1 + (imm << 16)
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movhi (&mut self, instruction: u16, bus: &Bus) {
        let reg1 = instruction as usize & 0x1F;
        let reg2 = (instruction as usize >> 5) & 0x1F;

        self.gprs[reg2] = self.getGPR(reg1) + ((self.nextHalfword(bus) as u32) << 16);
    }

    // reg2 = reg1 + (sign extend) imm
    // Cycles: 1
    // Flags affected: none
    // Opcode: 0b101111
    pub fn movea (&mut self, instruction: u16, bus: &Bus) {
        let reg1 = instruction as usize & 0x1F;
        let reg2 = (instruction as usize >> 5) & 0x1F;

        self.gprs[reg2] = self.getGPR(reg1) + signExtendHalfword(self.nextHalfword(bus));
    }
}
use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    pub fn bcond(&mut self, bus: &mut Bus, instr: u16) {
        todo!("BCOND");
    }

    // pc = reg1
    // Cycles: 3
    // Flags affected: none
    // Opcode: 0b000110
    pub fn jmp(&mut self, bus: &mut Bus, instr: u16) {
        let reg1_idx = (instr & 0x1F) as usize;
        self.regs.pc = self.regs.gprs[reg1_idx] & !1;
    }
}

use crate::cpu::CPU;
use crate::bus::Bus;

impl CPU {
    // pc = reg1
    // Cycles: 3
    // Flags affected: none
    // Opcode: 0b000110
    pub fn jmp (&mut self, instruction: u16) {
        self.pc = self.getGPR(instruction as usize & 0x1F)
    }
}
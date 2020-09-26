use crate::cpu::Cpu;

impl Cpu {
    // pc = reg1
    // Cycles: 3
    // Flags affected: none
    // Opcode: 0b000110
    pub fn jmp (&mut self, instr: u16) {
        let reg1_idx = (instr & 0x1F) as usize;
        self.regs.pc = self.regs.gprs[reg1_idx] & !1;
    }
}

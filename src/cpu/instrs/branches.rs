use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    // if (cond): pc += (sign extend) offset
    // Cycles: 1 if branch not taken, 3 if taken
    // Flags affected: none
    // Opcode: 0b100xxx
    pub fn bcond(&mut self, bus: &mut Bus, instr: u16) {
        let cond = (instr >> 9) & 0xF;

        if self.regs.psw.satisfies_cond(cond) {
            let offset = ((instr & 0x1FF) as i32) << 23 >> 23;
            let offset = offset as u32;

            self.regs.pc = self.regs.pc.wrapping_sub(2).wrapping_add(offset);  // Calculate the new PC. Branch is relevant to the FIRST INSTRUCTION byte, hence the -2
        }
    }

    // pc = reg1
    // Cycles: 3
    // Flags affected: none
    // Opcode: 0b000110
    pub fn jmp(&mut self, bus: &mut Bus, instr: u16) {
        let reg1_index = (instr & 0x1F) as usize;
        self.regs.pc = self.regs.gprs[reg1_index] & !1;
    }

    
    // pc += (sign extend) offset
    // Cycles: 3
    // Flags affected: none
    // Opcode: 0b101010
    pub fn jr(&mut self, bus: &mut Bus, instr: u16) {
        let mut offset = (instr as u32 & 0x3FF) << 16;
        offset |= self.consume_halfword(bus) as u32;
    
        offset = ((offset as i32) << 6 >> 6) as u32;
        let addr = self.regs.pc.wrapping_sub(4).wrapping_add(offset) & !1; // Calculate the new PC. Branch is relevant to the FIRST INSTRUCTION byte, hence the -4
        self.regs.pc = addr;
    }
}

use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    // reg2 = (sign extend) [reg1 + (sign extend) offset]
    // Cycles:    
    //  1 cycle	    When used immediately after an instruction that takes many cycles [note] and which does not conflict with the operation of the load instruction.
    //  4 cycles	When immediately following another load instruction.
    //  5 cycles	When used in an isolated context.
    // Flags affected: none
    // Opcode: 0b000110
    pub fn ld_byte (&mut self, bus: &Bus, instr: u16) {
        let reg1_index = instr as usize & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;

        let offset = self.consume_halfword(bus) as i16 as u32; // sign extend offset
        let addr = self.regs.gprs[reg1_index].wrapping_add(offset);

        self.regs.gprs[reg2_index] = bus.read8(addr) as i8 as u32; // read byte, sign extend it
    }
}
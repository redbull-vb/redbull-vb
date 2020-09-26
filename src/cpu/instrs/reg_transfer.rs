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

    pub fn mov_imm(&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let imm = ((instr as i32) << 27 >> 27) as u32; // sign extend immediate

        self.regs.gprs[reg2_index] = imm;
    }

    pub fn mov_reg(&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg1_index = (instr & 0x1F) as usize;

        self.regs.gprs[reg2_index] = self.regs.gprs[reg1_index];
    }

    pub fn ldsr(&mut self, bus: &mut Bus, instr: u16) {
        let system_reg_id = instr & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let reg2 = self.regs.gprs[reg2_index];

        match system_reg_id {
            5 => { self.regs.psw.set_raw(reg2); println!("Wrote {:08X} to PSW", reg2) },
            24 => println!("Unimplemented write to Cache Control! Value {:08X}", reg2),
            _ => panic!("LDSR with unimplemented system register. Register id: {}", system_reg_id)
        }
    }
}

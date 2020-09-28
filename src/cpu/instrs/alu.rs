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

    pub fn addi_long (&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg1_index = (instr & 0x1F) as usize;

        let reg1 = self.regs.gprs[reg1_index];
        let imm = self.consume_halfword(bus) as i16 as u32;

        let (res, overflow) = (reg1 as i32).overflowing_add(imm as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(res < reg1); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);

        self.regs.gprs[reg2_index] = res;
    }

    pub fn add_reg (&mut self, bus: &mut Bus, instr: u16) {
        let reg2_index = (instr >> 5 & 0x1F) as usize;
        let reg1_index = (instr & 0x1F) as usize;

        let reg1 = self.regs.gprs[reg1_index];
        let reg2 = self.regs.gprs[reg2_index];

        let (res, overflow) = (reg1 as i32).overflowing_add(reg2 as i32);
        let res = res as u32;
        self.regs.psw.set_sign_and_zero(res);
        self.regs.psw.set_carry(res < reg1); // Set carry if the result wrapped around.
        self.regs.psw.set_overflow(overflow);

        self.regs.gprs[reg2_index] = res;
    }

    // NOTE: ANDI DOESN'T SIGN EXTEND
    pub fn andi (&mut self, bus: &mut Bus, instr: u16) {
        let reg1_index = instr as usize & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let imm = self.consume_halfword(bus);
        let reg1 = self.regs.gprs[reg1_index];
        let res = reg1 & imm as u32; 

        self.regs.psw.set_sign(false);
        self.regs.psw.set_overflow(false);
        self.regs.psw.set_zero(res == 0);
        self.regs.gprs[reg2_index] = res;
    }

     // NOTE: ORI DOESN'T SIGN EXTEND
     pub fn ori (&mut self, bus: &mut Bus, instr: u16) {
        let reg1_index = instr as usize & 0x1F;
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let imm = self.consume_halfword(bus);
        let reg1 = self.regs.gprs[reg1_index];
        let res = reg1 | imm as u32; 

        self.regs.psw.set_overflow(false);
        self.regs.psw.set_sign_and_zero(res);
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

    pub fn div (&mut self, instr: u16) {
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let reg1 = self.regs.gprs[instr as usize & 0x1F];
        let reg2 = self.regs.gprs[reg2_index];

        let (res, overflow) = (reg2 as i32).overflowing_div(reg1 as i32);
        self.regs.gprs[30] = reg2 % reg1; // Reg2 MOD reg1 is stored in r30 during a DIV instruction
        self.regs.gprs[reg2_index] = res as u32;

        self.regs.psw.set_sign_and_zero(res as u32);
        self.regs.psw.set_overflow(overflow);
    }

    pub fn mul (&mut self, instr: u16) {
        let reg2_index = (instr as usize >> 5) & 0x1F;
        let reg1 = self.regs.gprs[instr as usize & 0x1F];
        let reg2 = self.regs.gprs[reg2_index];

        
        let (res, overflow) = (reg2 as i32 as i64).overflowing_mul(reg1 as i32 as i64);
        self.regs.gprs[30] = (res >> 32) as u32; // MUL is a 64-bit signed multiplication. Upper 32 bits are stored in r30
        self.regs.gprs[reg2_index] = res as u32; // Lower 32 bits are stored in reg2
    }
}

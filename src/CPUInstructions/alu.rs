use crate::cpu::CPU;
use crate::bus::Bus;
use crate::helpers::signExtendHalfword;

impl CPU {
    // reg2 = reg2 + (sign extend) imm
    // Cycles: 1
    // Flags affected: Zero, Sign, Carry, Overflow
    // Opcode: 0b010001
    pub fn addi_short (&mut self, instruction: u16) {
        let reg2Index = (instruction as usize >> 5) & 0x1F;
        let reg2 = self.getGPR(reg2Index);
        let mut imm = instruction as u32 & 0x1F;

        if (imm >> 4) == 1 { // Sign extend the immediate
            imm |= 0xFFFFFFE0;
        }

        let res = reg2 + imm;
        self.setSignAndZero(res);
        self.psw.setCarry((res < reg2) as u32); // Set carry if the result wrapped around. IE if it became bigger than U32_MAX and ended up smaller than the 2 operands
        self.psw.setOverflow(((reg2 ^ res as u32) & (imm ^ res as u32)) >> 31); // Branchless signed overflow calculation formula

        self.gprs[reg2Index] = res;
    }
}
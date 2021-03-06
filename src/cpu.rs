mod instrs;
use crate::bus::Bus;
use instrs::opcodes;

bitfield! {
    pub struct Psw(u32);
    
    pub raw, set_raw: 31, 0;
    // CPU control flags

    // "Interrupt level". If an interrupt is requested and its level is less than I, it gets masked
    pub i, set_i: 19, 16;
    // "NMI pending" flag. Shows if a non-maskable interrupt is pending
    pub nmi_pending, set_nmi_pending: 15;
    // Shows if there's a pending exception, set during exception processing or interrupts
    pub exception_pending, set_exception_pending: 14;
    pub addr_trap_enabled, set_addr_trap_enabled: 13;
    // Shows if interrupts are disabled
    pub irqs_disabled, set_irqs_disabled: 12;

    // Floating point flags

    // Floating reserved operand flags. Set when a FP instruction is attempted with a reserved operand.
    pub fro, set_fro: 9;
    // Set when an invalid FP op is attempted
    pub fiv, set_fiv: 8;
    // Set when a DIVF.S instruction is executed with a divisor of 0
    pub fzd, set_fzd: 7;
    // Set when the result of a floating-point operation is too large to be represented by the floating short data type.
    pub fov, set_fov: 6;
    // Set when the result of a floating-point operation is too small to be represented as a normal floating short value.
    pub fud, set_fud: 5;
    // Set when the result of a floating-point operation is subjected to rounding and suffers precision degradation.
    pub fpr, set_fpr: 4;

    // Actually useful flags

    // Set when an op produces a carry
    pub carry, set_carry: 3;
    // Set when an op produces an integer overflow
    pub overflow, set_overflow: 2;
    // Set to the sign bit (most significant bit) of the result of an operation
    pub sign, set_sign: 1;
    // Set if the result of an operation is 0
    pub zero, set_zero: 0;
}

impl Psw {
    // Helper function that sets the zero and sign flags depending on the result of an op.
    // Zero: Set if the result is 0
    // Sign: Set to the sign bit (bit 31) of the result
    pub fn set_sign_and_zero(&mut self, num: u32) {
        self.set_sign(num >> 31 != 0);
        self.set_zero(num == 0);
    }

    // Parameters: A condition code (0 - 15)
    // Returns: Whether the condition is true, depending on the current PSW 
    pub fn satisfies_cond(&self, cond: u16) -> bool {
        debug_assert!(cond < 16);

        match cond {
            0 => self.overflow(),                                   // V
            1 => self.carry(),                                      // C
            2 => self.zero(),                                       // E
            3 => self.zero() || self.carry(),                       // NH
            4 => self.sign(),                                       // N
            5 => true,                                              // Always (BR)
            6 => self.overflow() || self.sign(),                    // LT
            7 => ((self.overflow() ^ self.sign()) || self.zero()),  // LE
            8 => !self.overflow(),                                  // MV
            9 => !self.carry(),                                     // NC
            10 => !self.zero(),                                     // NE
            11 => !(self.zero() || self.carry()),                   // H
            12 => !self.sign(),                                     // P
            13 => false,                                            // Never (NOP)
            14 => !(self.overflow() || self.sign()),                // GT
            _ => !((self.overflow() ^ self.sign()) || self.zero()), // GE
        }
    }
}

pub struct Regs {
    pub gprs: [u32; 32], // CPU general purpose registers (r0-r31)
    pub pc: u32,         // program counter
    pub psw: Psw,        // CPU flags
}

pub struct Cpu {
    pub regs: Regs,
    // TODO: Add the different system registers, accessible via instructions LDSR and STSR
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Regs {
                gprs: [0; 32],
                pc: 0xFFFFFFF0,       // PC value on reset
                psw: Psw(0x00008000), // PSW value on reset
            },
        }
    }

    // Step the CPU by one instruction
    pub fn step(&mut self, bus: &mut Bus) {
        if self.regs.pc == 0x7001CC0 {panic!("breakpoint")}

        let instr = bus.read16(self.regs.pc); // Fetch an opcode. Opcodes are fetched halfword-by-halfword and can be 16 or 32 bits
        let opcode = instr >> 10; // Top 6 bits of each instruction determines its type.
        self.regs.pc = self.regs.pc.wrapping_add(2); // Increment PC

        //println!("{}", instrs::disassembler::disassemble(self, bus, instr, &mut self.regs.pc.clone()));

        match opcode {
            opcodes::BCOND_START..=opcodes::BCOND_END => self.bcond(bus, instr),
            opcodes::JMP => self.jmp(bus, instr), // JMP reg
            opcodes::JR  => self.jr(bus, instr), // JR $addr
            opcodes::JAL => self.jal(bus, instr), // JAL $addr

            opcodes::MOVEA => self.movea(bus, instr), // MOVEA
            opcodes::MOVHI => self.movhi(bus, instr), // MOVHI
            opcodes::MOV_IMM => self.mov_imm(bus, instr), // mov reg2, #imm
            opcodes::MOV_REG => self.mov_reg(bus, instr), // mov reg2, reg1

            opcodes::ADD_REG => self.add_reg(bus, instr), // ADD reg2, reg1
            opcodes::ADDI_SHORT => self.addi_short(bus, instr), // ADD reg2, #imm. 16-bit version of ADDI.
            opcodes::ADDI_LONG => self.addi_long(bus, instr), // ADDI reg2, reg1, #imm with a 32-bit imm.
            opcodes::ANDI => self.andi(bus, instr), // andi r2, r1, (zero extend) #imm
            opcodes::ORI => self.ori(bus, instr), // ori r2, r1, (zero extend) #imm
            opcodes::CMP_IMM => self.cmp_imm(bus, instr), // cmp reg2, #imm
            opcodes::CMP_REG => self.cmp_reg(bus, instr), // cmp reg2, reg1
            opcodes::DIV => self.div(instr), // r30 = reg2 MOD reg1. reg2 = reg2 / reg1.
            opcodes::MUL => self.mul(instr), // res = (signed) reg2 * (signed) reg1. r30 = (res >> 32). reg2 = (reg & 0xFFFFFFFF)

            opcodes::LD_BYTE => self.ld_byte(bus, instr), // reg2 = (byte) [reg1 + disp]
            opcodes::LD_HALFWORD => self.ld_halfword(bus, instr), // reg2 = (halfword) [reg1 + disp]
            opcodes::LD_WORD => self.ld_word(bus, instr), // reg2 = (word) [reg1 + disp]
            opcodes::ST_BYTE => self.st_byte(bus, instr), // [reg1 + disp] = reg2 & 0xFF
            opcodes::ST_HALFWORD => self.st_halfword(bus, instr), // [reg1 + disp] = reg2 & 0xFFFF
            opcodes::ST_WORD => self.st_word(bus, instr), // [reg1 + disp] = reg2

            opcodes::LDSR => self.ldsr(bus, instr), // systemReg = reg2

            opcodes::SEI => self.sei(), // interrupts disabled = true;

            _ => panic!("Unimplemented opcode {:b} at address {:08X}", opcode, self.regs.pc.wrapping_sub(2)),
        }

        self.regs.gprs[0] = 0;
    }

    // Read 2 bytes from mem[pc] and increment PC
    pub fn consume_halfword(&mut self, bus: &Bus) -> u16 {
        let val = bus.read16(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(2);

        val
    }

    // Read 4 bytes from mem[pc] and increment PC
    pub fn consume_word(&mut self, bus: &Bus) -> u32 {
        let val = bus.read32(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(4);

        val
    }
}

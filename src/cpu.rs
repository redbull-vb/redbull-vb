mod instrs;

use crate::bus::Bus;

const JMP_OPCODE:   u16 = 0b000110;
const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;

const ADDI_SHORT_OPCODE: u16 = 0b010001;

bitfield!{
    pub struct Psw(u32);

    // CPU control flags

    /// "Interrupt level". If an interrupt is requested and its level is less than I, it gets masked
    pub i, set_i: 19, 16;
    /// "NMI pending" flag. Shows if a non-maskable interrupt is pending
    pub nmi_pending, set_nmi_pending: 15;
    /// Shows if there's a pending exception, set during exception processing or interrupts
    pub exception_pending, set_exception_pending: 14;
    pub addr_trap_enabled, set_addr_trap_enabled: 13;
    /// Shows if interrupts are disabled
    pub irqs_disabled, set_irqs_disabled: 12;

    // Floating point flags

    /// Floating reserved operand flags. Set when a FP instruction is attempted with a reserved operand.
    pub fro, set_fro: 9;
    /// Set when an invalid FP op is attempted
    pub fiv, set_fiv: 8;
    /// Set when a DIVF.S instruction is executed with a divisor of 0
    pub fzd, set_fzd: 7;
    /// Set when the result of a floating-point operation is too large to be represented by the floating short data type.
    pub fov, set_fov: 6;
    /// Set when the result of a floating-point operation is too small to be represented as a normal floating short value.
    pub fud, set_fud: 5;
    /// Set when the result of a floating-point operation is subjected to rounding and suffers precision degradation.
    pub fpr, set_fpr: 4;

    // Actually useful flags

    /// Set when an op produces a carry
    pub carry, set_carry: 3;
    /// Set when an op produces an integer overflow
    pub overflow, set_overflow: 2;
    /// Set to the sign bit (most significant bit) of the result of an operation
    pub sign, set_sign: 1; 
    /// Set if the result of an operation is 0
    pub zero, set_zero: 0;
}

impl Psw {
    /// Helper function that sets the zero and sign flags depending on the result of an op.
    /// Zero: Set if the result is 0
    /// Sign: Set to the sign bit (bit 31) of the result
    pub fn set_sign_and_zero(&mut self, num: u32) {
        self.set_sign(num >> 31 != 0);
        self.set_zero(num == 0);
    }
}

pub struct Regs {
    pub gprs: [u32; 32], // CPU general purpose registers (r0-r31)
    pub pc: u32, // program counter
    pub psw: Psw // CPU flags
}

pub struct Cpu {
    pub regs: Regs,
    // TODO: Add the different system registers, accessible via instructions LDSR and STSR
}

impl Cpu {
    pub fn new () -> Cpu {
        Cpu {
            regs: Regs {
                gprs: [0; 32],
                pc: 0xFFFFFFF0, // PC value on reset
                psw: Psw(0x00008000) // PSW value on reset
            }
        }
    }

    /// Step the CPU by one instruction
    pub fn step (&mut self, bus: &mut Bus) {
        let instruction = bus.read16(self.regs.pc); // Fetch an opcode. Opcodes are fetched halfword-by-halfword and can be 16 or 32 bits
        let opcode = instruction >> 10; // Top 6 bits of each instruction determines its type.
        self.regs.pc = self.regs.pc.wrapping_add(2); // Increment PC

        println!("{}", instrs::disassembler::disassemble(self, bus, instruction, &mut self.regs.pc.clone()));

        match opcode {
            JMP_OPCODE   => self.jmp(instruction), // JMP

            MOVEA_OPCODE => self.movea(instruction, bus), // MOVEA
            MOVHI_OPCODE => self.movhi(instruction, bus), // MOVHI

            ADDI_SHORT_OPCODE => self.addi_short(instruction), // ADD r2, #imm. 16-bit version of ADDI.
            
            _ => panic!("Unimplemented opcode {:b} at address {:08X}", opcode, self.regs.pc.wrapping_sub(2)),
        }

        self.regs.gprs[0] = 0;
    }

    /// Read 2 bytes from mem[pc] and increment PC
    pub fn consume_halfword (&mut self, bus: &Bus) -> u16 {
        let val = bus.read16(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(2);

        val
    }

    /// Read 4 bytes from mem[pc] and increment PC
    pub fn consume_word (&mut self, bus: &Bus) -> u32 {
        let val = bus.read32(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(4);

        val
    }
}

extern crate bitfield;
use bitfield::bitfield;
use crate::bus::Bus;

bitfield!{
    pub struct PSW(u32);
    pub getRaw, setRaw: 31, 0;

    // CPU control flags

    pub getI, setI: 19, 16; // "Interrupt level". If an interrupt is requested and its level is less than I, it gets masked
    pub getNMIPending, setNMIPending: 15, 15; // "NMI pending" flag. Shows if a non-maskable interrupt is pending
    pub getExceptionPending, setExceptionPending: 14, 14; // Shows if there's a pending exception, set during exception processing or interrupts
    pub getAddrTrapEnable, setAddrTrapEnable: 13, 13;
    pub getIRQDisable, setIRQDisable: 12, 12; // Shows if interrupts are disabled

    // Floating point flags

    pub getFRO, setFRO: 9, 9; // Floating reserved operand flags. Set when a FP instruction is attempted with a reserved operand.
    pub getFIV, setFIV: 8, 8; // Set when an invalid FP op is attempted
    pub getFZD, setFZD: 7, 7; // Set when a DIVF.S instruction is executed with a divisor of 0
    pub getFOV, setFOV: 6, 6; // Set when the result of a floating-point operation is too large to be represented by the floating short data type.
    pub getFUD, setFUD: 5, 5; // Set when the result of a floating-point operation is too small to be represented as a normal floating short value.
    pub getFPR, setFPR: 4, 4; // Set when the result of a floating-point operation is subjected to rounding and suffers precision degradation.

    // Actually useful flags

    pub getCarry, setCarry: 3, 3; // Set when an op produces a carry
    pub getOverflow, setOverflow: 2, 2; // Set when an op produces an integer overflow
    pub getSign, setSign: 1, 1;  // Set to the sign bit (most significant bit) of the result of an operation
    pub getZero, setZero: 0, 0; // Set if the result of an operation is 0
}

pub struct CPU {
    gprs: [u32; 32], // CPU general purpose registers (r0-r31)
    pc: u32, // program counter
    psw: PSW // CPU flags
             // TODO: Add the different system registers, accessible via instructions LDSR and STSR
}

impl CPU {
    pub fn new () -> CPU {
        CPU {
            gprs: [0; 32],
            pc: 0xFFFFFFF0, // PC value on reset
            psw: PSW(0x00008000) // PSW value on reset
        }
    }

    pub fn step (&mut self, bus: &mut Bus) {
        let opcode = bus.read16(self.pc); // Fetch an opcode. Opcodes are fetched halfword-by-halfword and can be 16 or 32 bits
        panic!("Unimplemented opcode {:04X} at {:08X}", opcode, self.pc & 0x7FFFFFF);
        self.pc += 2; // Increment PC
    }
}
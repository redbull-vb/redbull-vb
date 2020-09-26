#[allow(unused)]
pub mod disassembler;

pub mod alu;
pub mod branches;
pub mod reg_transfer;

pub mod opcodes {
    pub const MOV_IMM: u16 = 0b010000;
    pub const MOV_REG: u16 = 0b000000;
    pub const MOVEA: u16 = 0b101000;
    pub const MOVHI: u16 = 0b101111;
    pub const ADD_IMM: u16 = 0b010001;
    pub const ADD_REG: u16 = 0b000001;
    pub const ADDI: u16 = 0b101001;
    pub const CMP_IMM: u16 = 0b010011;
    pub const CMP_REG: u16 = 0b000011;
    pub const DIV: u16 = 0b001001;
    pub const DIVU: u16 = 0b001011;
    pub const MUL: u16 = 0b001000;
    pub const MULU: u16 = 0b001010;
    pub const SUB: u16 = 0b000010;
    pub const BCOND_START: u16 = 0b100000;
    pub const BCOND_END: u16 = 0b100111;
    pub const JAL: u16 = 0b101011;
    pub const JMP: u16 = 0b000110;
    pub const JR: u16 = 0b101010;
}

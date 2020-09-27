#[allow(unused)]
pub mod disassembler;

pub mod alu;
pub mod branches;
pub mod reg_transfer;
pub mod loads_stores;

pub mod opcodes {
    pub const MOV_IMM: u16 = 0b010000;
    pub const MOV_REG: u16 = 0b000000;
    pub const MOVEA: u16 = 0b101000;
    pub const MOVHI: u16 = 0b101111;

    pub const ADD_REG: u16 = 0b000001;
    pub const ADDI_LONG: u16 = 0b101001; // 32-bit version of ADDI
    pub const ADDI_SHORT: u16 = 0b010001; // 16-bit version of ADDI
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

    pub const IN_BYTE: u16 = 0b111000;
    pub const IN_HALFWORD: u16 = 0b111001;
    pub const IN_WORD: u16 = 0b111011;

    pub const LD_BYTE: u16 = 0b110000;
    pub const ST_BYTE: u16 = 0b110100; 
    pub const LD_HALFWORD: u16 = 0b110001;
    pub const ST_HALFWORD: u16 = 0b110101;
    pub const LD_WORD: u16 = 0b110011;
    pub const ST_WORD: u16 = 0b110111;

    pub const LDSR: u16 = 0b011100;
}

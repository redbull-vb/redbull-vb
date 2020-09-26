use crate::cpu::CPU;
use crate::bus::Bus;
use crate::helpers::signExtendHalfword;

const JMP_OPCODE:   u16 = 0b000110;
const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;

pub fn disassemble(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let opcode = instruction >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        JMP_OPCODE   => disassembleJMP(instruction, cpu, bus),
        MOVEA_OPCODE => disassembleMOVEA(instruction, cpu, bus),
        MOVHI_OPCODE => disassembleMOVHI(instruction, cpu, bus),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instruction)
    }
}

pub fn disassembleJMP(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    format!("jmp r{}", reg1)
}

pub fn disassembleMOVEA(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc);

    format!("movea r{}, r{} + 0x{:08X}", reg2, reg1, signExtendHalfword(imm))
}

pub fn disassembleMOVHI(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc) as u32;

    format!("movhi r{}, r{} + 0x{:08X}", reg2, reg1, imm << 16)
}
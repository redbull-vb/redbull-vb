use crate::cpu::CPU;
use crate::bus::Bus;

const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;

pub fn disassemble(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let opcode = instruction >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        MOVHI_OPCODE => disassembleMOVHI(instruction, cpu, bus),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instruction)
    }
}

pub fn disassembleMOVHI(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc) as u32;

    format!("movhi r{}, r{} + 0x{:08X}", reg2, reg1, imm << 16)
}
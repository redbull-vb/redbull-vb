use crate::cpu::CPU;
use crate::bus::Bus;
use crate::helpers::signExtendHalfword;

const JMP_OPCODE:   u16 = 0b000110;
const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;
const MOVREG_OPCODE: u16 = 0b000000;
const MOVIMM_OPCODE: u16 = 0b010000;
const ADDREG_OPCODE: u16 = 0b000001;
const ADDIMM_OPCODE: u16 = 0b010001;
const ADDI_OPCODE: u16 = 0b101001;

pub fn disassemble(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let opcode = instruction >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        JMP_OPCODE   => disassembleJMP(instruction, cpu, bus),
        MOVEA_OPCODE => disassembleMOVEA(instruction, cpu, bus),
        MOVHI_OPCODE => disassembleMOVHI(instruction, cpu, bus),
        MOVIMM_OPCODE => disassembleMOVimm(instruction, cpu, bus),
        MOVREG_OPCODE => disassembleMOVreg(instruction, cpu, bus),
        ADDREG_OPCODE => disassembleADDreg(instruction, cpu, bus),
        ADDIMM_OPCODE => disassembleADDimm(instruction, cpu, bus),
        ADDI_OPCODE => disassembleADDI(instruction, cpu, bus),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instruction)
    }
}

pub fn disassembleJMP(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    format!("jmp r{}", reg1)
}

pub fn disassembleMOVimm(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc);
    format!("mov r{}, 0x{:08X}", reg2, signExtendHalfword(imm))
}

pub fn disassembleMOVreg(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    format!("mov r{}, r{}", reg2, reg1)
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

pub fn disassembleADDreg(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    format!("add r{}, r{}", reg2, reg1)
}

pub fn disassembleADDimm(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc);
    format!("add r{}, 0x{:08X}", reg2, signExtendHalfword(imm))
}

pub fn disassembleADDI(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc);
    format!("add r{}, r{} + 0x{:08X}", reg2, reg1, signExtendHalfword(imm))
}
use crate::cpu::CPU;
use crate::bus::Bus;
use crate::helpers::signExtendHalfword;

const JMP_OPCODE:   u16 = 0b000110;
const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;
const ADDI_SHORT_OPCODE: u16 = 0b010001;

const CONDITION_CODES: &[&str] = &["v", "c", "e", "nh", "n", "r", "lt", "le", "nv", "nc", "ne", "h", "p", "nop", "ge", "gt"];

pub fn disassemble(instruction:u16, cpu: &CPU, bus: &Bus) -> String {
    let opcode = instruction >> 10; // Top 6 instruction bits decide the type of instruction

    if (instruction >> 13) == 0b100 { // Special case BCOND, as it doesn't follow the normal instruction format
        return disassembleBCOND (instruction)
    }

    match opcode {
        JMP_OPCODE   => disassembleJMP(instruction),
        MOVEA_OPCODE => disassembleMOVEA(instruction, cpu, bus),
        MOVHI_OPCODE => disassembleMOVHI(instruction, cpu, bus),
        ADDI_SHORT_OPCODE => disassembleADDIShort (instruction, cpu),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instruction)
    }
}

pub fn disassembleBCOND(instruction: u16) -> String {
    let cond = (instruction as usize >> 9) & 0xF;
    let mut disp = instruction as u32 & 0x1FF; // Displacement
    if (disp >> 8) == 1 { // sign extend displacement
        disp |= 0xFFFFFE00;
    }

    format!("b{} {}", CONDITION_CODES[cond], disp as i32)
}

pub fn disassembleJMP(instruction: u16) -> String {
    let reg1 = instruction & 0x1F;
    format!("jmp r{}", reg1)
}

pub fn disassembleMOVEA(instruction: u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc);

    format!("movea r{}, r{} + 0x{:08X}", reg2, reg1, signExtendHalfword(imm))
}

pub fn disassembleMOVHI(instruction: u16, cpu: &CPU, bus: &Bus) -> String {
    let reg1 = instruction & 0x1F;
    let reg2 = (instruction >> 5) & 0x1F;
    let imm = bus.read16(cpu.pc) as u32;

    format!("movhi r{}, r{} + 0x{:08X}", reg2, reg1, imm << 16)
}

pub fn disassembleADDIShort(instruction: u16, cpu: &CPU) -> String {
    let reg2 = (instruction as usize >> 5) & 0x1F;
    let mut imm = instruction as u32 & 0x1F;

    if (imm >> 4) == 1 { // Sign extend the immediate
        imm |= 0xFFFFFFE0;
    }

    format!("addi r{}, 0x{:08X}", reg2, imm)
}
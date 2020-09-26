use crate::cpu::Cpu;
use crate::bus::Bus;

const JMP_OPCODE:   u16 = 0b000110;
const MOVEA_OPCODE: u16 = 0b101000;
const MOVHI_OPCODE: u16 = 0b101111;
const ADDI_SHORT_OPCODE: u16 = 0b010001;

pub fn disassemble(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let opcode = instr >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        JMP_OPCODE   => disassemble_jmp(cpu, bus, instr, pc),
        MOVEA_OPCODE => disassemble_movea(cpu, bus, instr, pc),
        MOVHI_OPCODE => disassemble_movhi(cpu, bus, instr, pc),
        ADDI_SHORT_OPCODE => disassemble_addi_short (cpu, bus, instr, pc),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instr)
    }
}

pub fn disassemble_jmp(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    format!("jmp r{}", reg1_idx)
}

pub fn disassemble_movea(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = bus.read16(*pc);
    *pc = pc.wrapping_add(2);

    format!("movea r{}, r{} + 0x{:08X}", reg2_idx, reg1_idx, imm as i16 as u32)
}

pub fn disassemble_movhi(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = bus.read16(*pc) as u32;
    *pc = pc.wrapping_add(2);

    format!("movhi r{}, r{} + 0x{:08X}", reg2_idx, reg1_idx, imm << 16)
}

pub fn disassemble_addi_short(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr as usize >> 5) & 0x1F;
    let imm = ((instr as i32) << 27 >> 27) as u32;

    format!("addi r{}, 0x{:08X}", reg2_idx, imm)
}

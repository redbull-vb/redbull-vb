use super::opcodes;
use crate::bus::Bus;
use crate::cpu::Cpu;

const CONDITION_CODES: &[&str] = &[
    "bv", "bc", "bz", "bnh", "bn", "br", "blt", "ble", "bnv", "bnc", "bnz", "bh", "bp", "nop",
    "bge", "bgt",
];

pub fn disassemble(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let opcode = instr >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        opcodes::BCOND_START..=opcodes::BCOND_END => disassemble_bcond(cpu, bus, instr, pc),
        opcodes::JMP => disassemble_jmp(cpu, bus, instr, pc),
        opcodes::MOV_IMM => disassemble_mov_imm(cpu, bus, instr, pc),
        opcodes::MOV_REG => disassemble_mov_reg(cpu, bus, instr, pc),
        opcodes::MOVEA => disassemble_movea(cpu, bus, instr, pc),
        opcodes::MOVHI => disassemble_movhi(cpu, bus, instr, pc),
        opcodes::ADD_IMM => disassemble_addi(cpu, bus, instr, pc),
        opcodes::ADD_REG => disassemble_add_reg(cpu, bus, instr, pc),
        opcodes::ADDI => disassemble_addi(cpu, bus, instr, pc),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instr),
    }
}

pub fn disassemble_mov_imm(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = bus.read16(cpu.regs.pc) as i16 as u32;
    *pc = pc.wrapping_add(2);

    format!("mov r{}, {:#010X}", reg2_idx, imm)
}

pub fn disassemble_mov_reg(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr >> 5) & 0x1F;
    let reg1_idx = instr & 0x1F;

    format!("mov r{}, r{}", reg2_idx, reg1_idx)
}

pub fn disassemble_movea(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = bus.read16(*pc) as i16 as i32;
    *pc = pc.wrapping_add(2);

    format!("movea r{}, r{} + {:#010X}", reg2_idx, reg1_idx, imm)
}

pub fn disassemble_movhi(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = (bus.read16(*pc) as u32) << 16;
    *pc = pc.wrapping_add(2);

    format!("movhi r{}, r{} + 0x{:08X}", reg2_idx, reg1_idx, imm)
}

pub fn disassemble_add_imm(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr >> 5) & 0x1F;
    let imm = bus.read16(cpu.regs.pc) as i16 as i32;
    *pc = pc.wrapping_add(2);

    format!("add r{}, {:#010X}", reg2_idx, imm)
}

pub fn disassemble_add_reg(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr >> 5) & 0x1F;
    let reg1_idx = instr & 0x1F;

    format!("add r{}, r{}", reg2_idx, reg1_idx)
}

pub fn disassemble_addi(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_idx = (instr as usize >> 5) & 0x1F;
    let imm = ((instr as i32) << 27 >> 27) as u32;

    format!("addi r{}, {:#010X}", reg2_idx, imm)
}

pub fn disassemble_bcond(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let cond = instr >> 9 & 0xF;
    let mut disp = (instr as i32) << 23 >> 23; // Displacement
    format!("{} {}", CONDITION_CODES[cond as usize], disp)
}

pub fn disassemble_jmp(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_idx = instr & 0x1F;
    format!("jmp r{}", reg1_idx)
}

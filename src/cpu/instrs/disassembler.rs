use super::opcodes;
use crate::bus::Bus;
use crate::cpu::Cpu;

const BCOND_MNEMONICS: &[&str] = &[
    "bv", "bc", "be", "bnh", "bn", "br", "blt", "ble", "bnv", "bnc", "bne", "bh", "bp", "nop",
    "bge", "bgt",
];

pub fn disassemble(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let opcode = instr >> 10; // Top 6 instruction bits decide the type of instruction

    match opcode {
        opcodes::BCOND_START..=opcodes::BCOND_END => disassemble_bcond(cpu, bus, instr, pc),
        opcodes::JMP => disassemble_jmp(cpu, bus, instr, pc),
        opcodes::JR => disassemble_jr(cpu, bus, instr, pc),
        
        opcodes::MOV_IMM => disassemble_mov_imm(cpu, bus, instr, pc),
        opcodes::MOV_REG => disassemble_mov_reg(cpu, bus, instr, pc),
        opcodes::MOVEA => disassemble_movea(cpu, bus, instr, pc),
        opcodes::MOVHI => disassemble_movhi(cpu, bus, instr, pc),

        opcodes::ADD_REG => disassemble_add_reg(cpu, bus, instr, pc),
        opcodes::ADDI_SHORT => disassemble_addi_short(cpu, bus, instr, pc),
        opcodes::CMP_REG => disassemble_cmp_reg(cpu, bus, instr, pc),
        
        opcodes::LD_BYTE => disassemble_ld(cpu, bus, instr, pc, "b".to_string()),
        opcodes::ST_BYTE => disassemble_st(cpu, bus, instr, pc, "b".to_string()),
        opcodes::LD_HALFWORD => disassemble_ld(cpu, bus, instr, pc, "h".to_string()),
        opcodes::ST_HALFWORD => disassemble_st(cpu, bus, instr, pc, "h".to_string()),
        opcodes::LD_WORD => disassemble_ld(cpu, bus, instr, pc, "w".to_string()),
        opcodes::ST_WORD => disassemble_st(cpu, bus, instr, pc, "w".to_string()),
        _ => panic!("[Disassembler]: Unrecognized instruction {:04X}", instr),
    }
}

pub fn disassemble_mov_imm(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr >> 5) & 0x1F;
    let imm = bus.read16(cpu.regs.pc) as i16 as u32;

    format!("mov r{}, {:#010X}", reg2_index, imm)
}

pub fn disassemble_mov_reg(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr >> 5) & 0x1F;
    let reg1_index = instr & 0x1F;

    format!("mov r{}, r{}", reg2_index, reg1_index)
}

pub fn disassemble_movea(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_index = instr & 0x1F;
    let reg2_index = (instr >> 5) & 0x1F;
    let imm = bus.read16(*pc) as i16 as i32;

    format!("movea r{}, r{} + {:#010X}", reg2_index, reg1_index, imm)
}

pub fn disassemble_movhi(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_index = instr & 0x1F;
    let reg2_index = (instr >> 5) & 0x1F;
    let imm = (bus.read16(*pc) as u32) << 16;

    format!("movhi r{}, r{} + 0x{:08X}", reg2_index, reg1_index, imm)
}

pub fn disassemble_add_imm(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr >> 5) & 0x1F;
    let imm = bus.read16(cpu.regs.pc) as i16 as i32;

    format!("add r{}, {:#010X}", reg2_index, imm)
}

pub fn disassemble_add_reg(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr >> 5) & 0x1F;
    let reg1_index = instr & 0x1F;

    format!("add r{}, r{}", reg2_index, reg1_index)
}

pub fn disassemble_addi_short(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr as usize >> 5) & 0x1F;
    let imm = ((instr as i32) << 27 >> 27) as u32;

    format!("addi r{}, {:#010X}", reg2_index, imm)
}

pub fn disassemble_cmp_reg(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg2_index = (instr >> 5) & 0x1F;
    let reg1_index = instr & 0x1F;

    format!("cmp r{}, r{}", reg2_index, reg1_index)
}

pub fn disassemble_bcond(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let cond = instr >> 9 & 0xF;
    let disp = (instr as i32) << 23 >> 23; // Displacement
    let addr = pc.wrapping_sub(2).wrapping_add(disp as u32);
    format!("{} {:08X}", BCOND_MNEMONICS[cond as usize], addr)
}

pub fn disassemble_jmp(cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let reg1_index = instr & 0x1F;
    format!("jmp r{}", reg1_index)
}

pub fn disassemble_jr (cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32) -> String {
    let mut offset = (instr as u32 & 0x3FF) << 16;
    offset |= bus.read16(*pc) as u32;

    offset = ((offset as i32) << 6 >> 6) as u32;
    let addr = pc.wrapping_sub(2).wrapping_add(offset) & !1; // Calculate the new PC,
    format!("jr {:08X}", addr)
}

pub fn disassemble_ld (cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32, suffix: String) -> String {
    let reg1_index = instr as usize & 0x1F;
    let reg2_index = (instr as usize >> 5) & 0x1F;

    let offset = bus.read16(*pc) as i16 as u32;

    format!("ld.{} r{}, [r{} + {:08X}]", suffix, reg2_index, reg1_index, offset)
}

pub fn disassemble_st (cpu: &Cpu, bus: &Bus, instr: u16, pc: &mut u32, suffix: String) -> String {
    let reg1_index = instr as usize & 0x1F;
    let reg2_index = (instr as usize >> 5) & 0x1F;

    let offset = bus.read16(*pc) as i16 as u32;

    format!("st.{} r{}, [r{} + {:08X}]", suffix, reg2_index, reg1_index, offset)
}
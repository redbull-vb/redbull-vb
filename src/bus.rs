use crate::mem::Memory;

/*
    The Virtual Boy memory bus is 27 bits wide and is organized by hardware component:

    0x00000000 - 0x00FFFFFF	VIP - Virtual Image Processor
    0x01000000 - 0x01FFFFFF	VSU - Virtual Sound Unit
    0x02000000 - 0x02FFFFFF	Miscellaneous Hardware
    0x03000000 - 0x03FFFFFF	Unmapped
    0x04000000 - 0x04FFFFFF	Game Pak Expansion
    0x05000000 - 0x05FFFFFF	WRAM
    0x06000000 - 0x06FFFFFF	Game Pak RAM
    0x07000000 - 0x07FFFFFF	Game Pak ROM
    0x08000000 - 0xFFFFFFFF	Mirroring of memory map
*/

pub struct Bus {
    memory: Memory
}

impl Bus {
    pub fn new (rom_path: &str) -> Bus {
        Bus {
            memory: Memory::new(rom_path)
        }
    }

    pub fn read16 (&self, mut addr: u32) -> u16 {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits (as well as the lowest bit due to alignment).
        addr &= 0x07FF_FFFE;

        match addr >> 24 & 7 { // The range to which the address belongs to depends on bits 24-27 of the addr
            7 => {
                // ROM range
                let rom_addr = addr as usize & self.memory.rom_mask;
                u16::from_le_bytes([self.memory.rom[rom_addr], self.memory.rom[rom_addr+1]])
            }
            _ => panic!("16-bit read from unimpl memory address {:08X}", addr)
        }
    }

    pub fn read32 (&self, mut addr: u32) -> u32 {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits (as well as the lowest 2 bits due to alignment).
        addr &= 0x07FF_FFFC;

        match addr >> 24 & 7 { // The range to which the address belongs to depends on bits 24-27 of the addr
            7 => {
                // ROM range
                let rom_addr = addr as usize & self.memory.rom_mask;
                u32::from_le_bytes([self.memory.rom[rom_addr], self.memory.rom[rom_addr+1], self.memory.rom[rom_addr+2], self.memory.rom[rom_addr+3]])
            }
            _ => panic!("32-bit read from unimpl memory address {:08X}", addr)
        }
    }
}

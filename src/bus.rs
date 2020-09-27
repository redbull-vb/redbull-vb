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
    memory: Memory,
}

impl Bus {
    pub fn new(rom_path: &str) -> Bus {
        Bus {
            memory: Memory::new(rom_path),
        }
    }

    pub fn read8(&self, mut addr: u32) -> u8 {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits
        addr &= 0x07FF_FFFF;

        match addr >> 24 & 7 {
            // The range to which the address belongs to depends on bits 24-27 of the addr
            5 => self.memory.ram[addr as usize & 0xFFFF], // Handle RAM mirroring
            7 => {
                // ROM range
                let rom_addr = addr as usize & self.memory.rom_mask;
                self.memory.rom[rom_addr]
            }
            _ => panic!("8-bit read from unimpl memory address {:08X}", addr),
        }
    }

    pub fn read16(&self, mut addr: u32) -> u16 {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits (as well as the lowest bit due to alignment).
        addr &= 0x07FF_FFFE;

        match addr >> 24 & 7 {
            // The range to which the address belongs to depends on bits 24-27 of the addr
            0 => {
                // ROM range
                let vip_addr = addr as usize & 0x7FFFF;
                u16::from_le_bytes([self.memory.vip_memory_stub[vip_addr], self.memory.vip_memory_stub[vip_addr + 1]])
            }

            7 => {
                // ROM range
                let rom_addr = addr as usize & self.memory.rom_mask;
                u16::from_le_bytes([self.memory.rom[rom_addr], self.memory.rom[rom_addr + 1]])
            }
            _ => panic!("16-bit read from unimpl memory address {:08X}", addr),
        }
    }

    pub fn read32(&self, mut addr: u32) -> u32 {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits (as well as the lowest 2 bits due to alignment).
        addr &= 0x07FF_FFFC;

        match addr >> 24 & 7 {
            // The range to which the address belongs to depends on bits 24-27 of the addr
            0 => {
                // VIP range
                let vip_addr = addr as usize & 0x7FFFF;
                println!("Stubbed 32-bit read from VIP memory!");
                u32::from_le_bytes([
                    self.memory.vip_memory_stub[vip_addr],
                    self.memory.vip_memory_stub[vip_addr + 1],
                    self.memory.vip_memory_stub[vip_addr + 2],
                    self.memory.vip_memory_stub[vip_addr + 3],
                ])
            }

            5 => {
                let ram_addr = addr as usize & 0xFFFF;
                u32::from_le_bytes([
                    self.memory.ram[ram_addr],
                    self.memory.ram[ram_addr + 1],
                    self.memory.ram[ram_addr + 2],
                    self.memory.ram[ram_addr + 3],
                ])
            }

            7 => {
                // ROM range
                let rom_addr = addr as usize & self.memory.rom_mask;
                u32::from_le_bytes([
                    self.memory.rom[rom_addr],
                    self.memory.rom[rom_addr + 1],
                    self.memory.rom[rom_addr + 2],
                    self.memory.rom[rom_addr + 3],
                ])
            }
            _ => panic!("32-bit read from unimpl memory address {:08X}", addr),
        }
    }

    pub fn write8(&mut self, mut addr: u32, val: u8) {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits
        addr &= 0x07FF_FFFF;

        match addr >> 24 & 7 { // The range to which the address belongs to depends on bits 24-27 of the addr
            0 => {
                self.memory.vip_memory_stub[addr as usize & 0x7FFFF] = val;
                println!("Unimplemented 8-bit write to VIP memory!")
            },
            5 => self.memory.ram[addr as usize & 0xFFFF] = val, // Handle RAM mirroring
            _ => panic!("8-bit write to unimpl memory address {:08X}", addr),
        }
    }

    pub fn write16(&mut self, mut addr: u32, val: u16) {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits
        addr &= 0x07FF_FFFF;

        match addr >> 24 & 7 {  // The range to which the address belongs to depends on bits 24-27 of the addr
            0 => {
                self.memory.vip_memory_stub[addr as usize & 0x7FFFF] = val as u8;
                self.memory.vip_memory_stub[(addr as usize + 1) & 0x7FFFF] = (val >> 8) as u8;
                println!("Unimplemented 16-bit write to VIP memory!")
            },

            5 => {
                self.memory.ram[addr as usize & 0xFFFF] = val as u8;
                self.memory.ram[(addr as usize + 1) & 0xFFFF] = (val >> 8) as u8;
            }

            _ => panic!("16-bit write to unimpl memory address {:08X}", addr)
        }
    }

    pub fn write32(&mut self, mut addr: u32, val: u32) {
        // Addresses are 27-bit on the VB, so we mask out the top 5 bits
        addr &= 0x07FF_FFFF;

        match addr >> 24 & 7 {
            // The range to which the address belongs to depends on bits 24-27 of the addr
            0 => {
                println!("Unimplemented 32-bit write to VIP memory!");
                self.memory.vip_memory_stub[addr as usize & 0x7FFFF] = val as u8;
                self.memory.vip_memory_stub[(addr as usize + 1) & 0x7FFFF] = (val >> 8) as u8;
                self.memory.vip_memory_stub[(addr as usize + 2) & 0x7FFFF] = (val >> 16) as u8;
                self.memory.vip_memory_stub[(addr as usize + 3) & 0x7FFFF] = (val >> 24) as u8;
            }

            1 => {
                println!("Unimplemented 32-bit write to VSU memory!");
                self.memory.vsu_memory_stub[addr as usize & 0x7FF] = val as u8;
                self.memory.vsu_memory_stub[(addr as usize + 1) & 0x7FF] = (val >> 8) as u8;
                self.memory.vsu_memory_stub[(addr as usize + 2) & 0x7FF] = (val >> 16) as u8;
                self.memory.vsu_memory_stub[(addr as usize + 3) & 0x7FF] = (val >> 24) as u8;
            }

            2 => {
                println!("Unimplemented 32-bit write to misc hardware")
            }

            5 => {
                self.memory.ram[addr as usize & 0xFFFF] = val as u8;
                self.memory.ram[(addr as usize + 1) & 0xFFFF] = (val >> 8) as u8;
                self.memory.ram[(addr as usize + 2) & 0xFFFF] = (val >> 16) as u8;
                self.memory.ram[(addr as usize + 3) & 0xFFFF] = (val >> 24) as u8;
            }

            _ => panic!("32-bit write to unimpl memory address {:08X}", addr)
        }
    }
}

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
    pub fn new (romPath: String) -> Bus {
        Bus {
            memory: Memory::new(romPath)
        }
    }

    pub fn read16 (&self, address: u32) -> u16 {
        debug_assert!((address & 1) == 0); // Assert that the address is aligned

        let mut maskedAddress = address as usize & 0x07FFFFFF; // Addresses are 27-bit on the VB, so we mask out the top 5 bits.
        let val: u16;
        
        match maskedAddress >> 24 { // The range to which the address belongs to depends on bits 24-27 of the addr
            7 => { // ROM range
                maskedAddress -= 0x7000000;
                maskedAddress &= self.memory.ROMMask; // Handle ROM mirroring
                val = u16::from_le_bytes([self.memory.ROM[maskedAddress], self.memory.ROM[maskedAddress+1]])
            }
            _ => panic!("16-bit read from unimpl memory address {:08X}", maskedAddress)
        }

        val
    }

    pub fn read32 (&self, address: u32) -> u32 {
        debug_assert!((address & 3) == 0); // Assert that the address is aligned 

        let mut maskedAddress = address as usize & 0x07FFFFFF; // Addresses are 27-bit on the VB, so we mask out the top 5 bits.
        let val: u32;
        
        match maskedAddress >> 24 { // The range to which the address belongs to depends on bits 24-27 of the addr
            7 => { // ROM range
                maskedAddress -= 0x7000000;
                maskedAddress &= self.memory.ROMMask; // Handle ROM mirroring
                val = u32::from_le_bytes([self.memory.ROM[maskedAddress], self.memory.ROM[maskedAddress+1], self.memory.ROM[maskedAddress+2], self.memory.ROM[maskedAddress+3]])
            }
            _ => panic!("32-bit read from unimpl memory address {:08X}", maskedAddress)
        }

        val
    }
}
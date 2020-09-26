pub struct Memory {
    // main. non-IO memory
    pub rom: Vec<u8>,
    pub rom_mask: usize // Mask to handle ROM read mirroring
}

impl Memory {
    pub fn new(rom_path: &str) -> Memory {
        let rom = std::fs::read(&rom_path).expect("couldn't find the specified ROM file");
        assert!(rom.len().is_power_of_two(), "the specified ROM's size is not a power of two");
        let rom_mask = rom.capacity() - 1;
        
        Memory {
            rom,
            rom_mask,
        }
    }
}

use crate::mem::Memory;

pub struct Bus {
    memory: Memory
}

impl Bus {
    pub fn new (romPath: String) -> Bus {
        Bus {
            memory: Memory::new(romPath)
        }
    }
}
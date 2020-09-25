use crate::cpu::CPU;
use crate::bus::Bus;

pub struct VirtualBoy {
    cpu: CPU,
    bus: Bus
}

impl VirtualBoy {
    pub fn new(romPath: String) -> VirtualBoy {
        VirtualBoy {
            cpu: CPU::new(),
            bus: Bus::new(romPath)
        }
    }
}
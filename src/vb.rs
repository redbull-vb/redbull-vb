use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct VirtualBoy {
    cpu: Cpu,
    bus: Bus,
}

impl VirtualBoy {
    pub fn new(rom_path: &str) -> VirtualBoy {
        VirtualBoy {
            cpu: Cpu::new(),
            bus: Bus::new(rom_path),
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }
}

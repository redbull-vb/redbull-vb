use crate::bus::Bus;
use crate::cpu::Cpu;

impl Cpu {
    pub fn sei (&mut self) {
        self.regs.psw.set_irqs_disabled(true);
    }
}
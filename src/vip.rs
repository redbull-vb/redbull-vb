use crate::mem::Memory;
use crate::bus::Bus;

pub struct IO {
    pub intpnd: usize,
    pub intenb: usize,
    pub intclr: usize,
    pub dpstts: usize,
    pub dpctrl: usize,
    pub brta: usize,
    pub brtb: usize,
    pub brtc: usize,
    pub rest: usize,
    pub frmcyc: usize,
    pub cta: usize,
    pub xpstts: usize,
    pub xpctrl: usize,
}

pub struct Vip {
    pub regs: IO,
}
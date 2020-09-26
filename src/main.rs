#![warn(clippy::all)]
#![allow(clippy::verbose_bit_mask, clippy::new_without_default)]

#[macro_use]
extern crate bitfield;

// Todo: Remove this from here
pub mod bus;
pub mod cpu;
pub mod mem;
mod vb;
pub use vb::VirtualBoy;

fn main() {
    let mut vb = VirtualBoy::new("ROMs/ScreenDemo1.vb");

    loop {
        vb.step();
    }
}

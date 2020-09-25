#![warn(clippy::all)]
#![allow(nonstandard_style)]

// Todo: Remove this from here
pub mod cpu;
pub mod mem;
pub mod bus;
pub mod virtualBoy;
pub mod helpers;

use crate::virtualBoy::VirtualBoy;

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*; // For future use

fn main() {
    let mut vb = VirtualBoy::new("ROMs/ScreenDemo1.vb".to_string());

    loop {
        vb.step();
    }
}
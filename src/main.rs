#![warn(clippy::all)]
#![allow(nonstandard_style)]

pub mod cpu;
use crate::cpu::CPU;

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*; // TODO: Not import the entire thing

fn main() {
    let cpu = CPU::new();
}
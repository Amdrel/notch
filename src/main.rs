extern crate rand;
extern crate byteorder;
extern crate sdl2;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;
mod memory;
mod graphics;
mod interconnect;
mod vm;

fn main() {
    if let Some(rom_file_name) = env::args().nth(1) {
        let rom = read_bin(rom_file_name);

        let mut vm = vm::VirtualMachine::new(rom);
        vm.run();
    } else {
        println!("noth {} a CHIP-8 Virtual Machine in Rust\n", env!("CARGO_PKG_VERSION"));
        println!("usage: {} <rom file>", env::args().nth(0).unwrap());
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

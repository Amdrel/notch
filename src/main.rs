extern crate byteorder;
extern crate rand;
extern crate sdl2;
extern crate time;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;
mod graphics;
mod input;
mod interconnect;
mod memory;
mod sound;
mod vm;

fn main() {
    // TODO: Use an actual argument parser when adding options in the future.
    if let Some(rom_file_name) = env::args().nth(1) {
        let rom = read_bin(rom_file_name);

        let mut vm = vm::VirtualMachine::new(rom);
        vm.run();
    } else {
        print_usage();
        std::process::exit(1);
    }
}

/// Prints the application name alongside the cargo version.
fn print_version() {
    println!("notch {}", env!("CARGO_PKG_VERSION"));
}

/// Prints usage information.
fn print_usage() {
    println!("Notch is a CHIP-8 virtual machine written in Rust.");
    println!("");
    println!("Usage: notch [ROM]");
    println!("");
    println!("To contribute or report bugs, please see:");
    println!("<https://github.com/Reshurum/notch>");
}

/// Reads a file into a vector of unsigned bytes.
fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

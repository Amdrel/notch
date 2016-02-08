extern crate byteorder;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod interpreter;
mod cpu;

fn main() {
    // TODO: Replace unwrap.
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);

    let mut interpreter = interpreter::Interpreter::new(rom);
    interpreter.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

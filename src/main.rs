extern crate byteorder;
extern crate getopts;
extern crate rand;
extern crate sdl2;
extern crate time;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use getopts::Options;

mod cpu;
mod graphics;
mod input;
mod interconnect;
mod memory;
mod sound;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Initialize the argument parser and parse them.
    let mut opts = Options::new();
    opts.optflag("v", "version", "print version information");
    opts.optflag("h", "help", "Print this message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(opts);
            std::process::exit(1);
        },
    };

    // Argument flags.
    if matches.opt_present("v") {
        print_version();
        return;
    }
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    // Assume the first free argument is the rom filename.
    let rom_file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(opts);
        std::process::exit(1);
    };

    let rom = read_bin(&rom_file_name);
    println!("Loading rom: {}", rom_file_name);

    // Initialize the virtual machine and boot the rom.
    let mut vm = vm::VirtualMachine::new(rom);
    vm.run();
}

/// Reads a file into a vector of unsigned bytes.
fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let filename = format!("{}", path.as_ref().display());

    // Open and read the file if it exists.
    match File::open(path) {
        Ok(ref mut file) => {
            file.read_to_end(&mut buffer).unwrap();
        },
        Err(why) => {
            println!("notch: cannot open '{}': {}", filename, why);
            std::process::exit(2);
        },
    };

    buffer
}

/// Prints the application name alongside the cargo version.
fn print_version() {
    println!("notch {}", env!("CARGO_PKG_VERSION"));
}

/// Prints usage information.
fn print_usage(opts: Options) {
    println!("Notch is a CHIP-8 virtual machine written in Rust.");
    println!("");
    println!("{}", opts.usage("Usage: notch [OPTIONS] ROM"));
    println!("To contribute or report bugs, please see:");
    println!("<https://github.com/Reshurum/notch>");
}

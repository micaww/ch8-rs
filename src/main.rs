use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod disassembler;
mod display;
mod cpu;
mod window;
mod keyboard;
mod speaker;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("You must specify the ROM file to run.");

    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();

    let mut cpu = cpu::Cpu::new();

    cpu.init();
    cpu.load_program(&buffer);

    window::create_window(cpu);
}

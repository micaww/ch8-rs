use std::io;
use std::io::prelude::*;
use std::fs::File;

mod disassembler;
mod display;
mod cpu;
mod window;
mod keyboard;

fn main() {
    let mut cpu = cpu::Cpu::new();

    let mut f = File::open("C:\\Users\\micah\\IdeaProjects\\ch8-rs\\roms\\pong.rom").unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();

    cpu.init();
    cpu.load_program(&buffer);

    window::create_window(cpu);
}

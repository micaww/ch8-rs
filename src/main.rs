use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::{thread, time};

mod disassembler;
mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();

    let mut f = File::open("C:\\Users\\micah\\IdeaProjects\\ch8-rs\\roms\\pong.rom").unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();

    cpu.load_program(&buffer);

    loop {
        cpu.tick();

        thread::sleep(time::Duration::from_secs(1));
    }
}

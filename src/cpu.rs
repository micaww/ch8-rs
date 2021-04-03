use crate::disassembler;
use crate::disassembler::OpCode;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use std::num::Wrapping;

const PROGRAM_OFFSET: usize = 512;
const CLOCK_FREQUENCY_HZ: u32 = 500;
const TIMER_FREQUENCY_HZ: u32 = 60;

pub struct Cpu {
    program_counter: usize,
    index: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    memory: [u8; 4096], // 4K of memory
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    next_tick: u128,
    next_timer_tick: u128
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            program_counter: PROGRAM_OFFSET,
            index: 0,
            stack: [0; 16],
            stack_pointer: 0,
            memory: [0; 4096],
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            next_tick: 0,
            next_timer_tick: 0
        }
    }

    pub fn load_program(&mut self, buffer: &[u8]) {
        // load program into memory starting at the program offset
        for i in 0..buffer.len() {
            self.memory[PROGRAM_OFFSET + i] = buffer[i];
        }
    }

    pub fn tick(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        if self.next_tick <= now {
            self.execute();
            self.next_tick = now + (1000 / CLOCK_FREQUENCY_HZ as u128);
        }

        if self.next_timer_tick <= now {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            self.next_timer_tick = now + (1000 / TIMER_FREQUENCY_HZ as u128);
        }
    }

    fn execute(&mut self) {
        let addr = self.program_counter;
        let word = u16::from_be_bytes([
            self.memory[addr],
            self.memory[addr + 1]
        ]);
        let opcode = disassembler::disassemble_word(word);

        println!("tick @ 0x{:x?} ({:x?}): {:?}", addr, word, opcode);

        self.advance();
        self.execute_opcode(opcode);
    }

    /// advance to the next instruction
    fn advance(&mut self) {
        self.program_counter += 2;
    }

    fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ClearDisplay => {}
            OpCode::Return => {
                // pops a return address from the stack, then jumps to it
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer] as usize;
            }
            OpCode::Jump(addr) => {
                // sets program counter without pushing return address
                self.program_counter = addr as usize;
            }
            OpCode::Call(addr) => {
                // pushes return address to the stack, then jumps to address
                self.stack[self.stack_pointer] = self.program_counter as u16;
                self.stack_pointer += 1;
                self.program_counter = addr as usize;
            }
            OpCode::SkipEqVal(x, val) => {
                // skips the next instruction if rX is equal to a value
                if self.registers[x as usize] == val {
                    self.advance();
                }
            }
            OpCode::SkipNotEqVal(x, val) => {
                // skips the next instruction if rX is not equal to a value
                if self.registers[x as usize] != val {
                    self.advance();
                }
            }
            OpCode::SkipEq(x, y) => {
                // skips the next instruction if rX is equal to rY
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.advance();
                }
            }
            OpCode::SetVal(x, val) => {
                // set a value into a register
                self.registers[x as usize] = val;
            }
            OpCode::AddVal(x, val) => {
                // adds a value to a register w/o carry flag
                self.registers[x as usize] = (Wrapping(self.registers[x as usize]) + Wrapping(val)).0;
            }
            OpCode::Copy(x, y) => {
                // copies a value from rY to rX
                self.registers[x as usize] = self.registers[y as usize];
            }
            OpCode::Or(x, y) => {
                // sets rX to bitwise OR of rX and rY
                self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
            }
            OpCode::And(x, y) => {
                // sets rX to bitwise AND of rX and rY
                self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
            }
            OpCode::Xor(x, y) => {
                // sets rX to bitwise XOR of rX and rY
                self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
            }
            OpCode::Add(x, y) => {
                // adds rY to rX and sets flag to 1 if there is a carry
            }
            OpCode::Subtract(x, y) => {
                // subtracts rY from rX and sets flag to 0 if there is a borrow
            }
            OpCode::ShiftRight(x) => {
                // stores LSB as flag, then shifts rX to the right once
                let val = self.registers[x as usize];

                self.registers[x as usize] = val >> 1;
                self.registers[0xF] = val & 1;
            }
            OpCode::Difference(x, y) => {
                // sets rX to rY minus rX and sets flag to 0 if there is a borrow
            }
            OpCode::ShiftLeft(x) => {
                // stores MSB as flag, then shifts rX to the left once
                let val = self.registers[x as usize];

                self.registers[x as usize] = val << 1;
                self.registers[0xF] = if (val & 0b1000_0000) > 0 {
                    1
                } else {
                    0
                };
            }
            OpCode::SkipNotEq(x, y) => {
                // skips the next instruction if rX is not equal to rY
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.advance();
                }
            }
            OpCode::SetIndex(addr) => {
                // sets index register
                self.index = addr;
            }
            OpCode::JumpOffset(addr) => {
                // jumps to address with an offset of r0
                self.program_counter = (addr + (self.registers[0] as u16)) as usize;
            }
            OpCode::Rand(x, val) => {}
            OpCode::DrawSprite(x, y, val) => {}
            OpCode::SkipKeyPressed(x) => {
                // skips next instruction if key at rX is pressed
            }
            OpCode::SkipKeyNotPressed(x) => {
                // skips next instruction if key at rX is not pressed
            }
            OpCode::GetDelayTimer(x) => {
                // sets rX to value of delay timer
                self.registers[x as usize] = self.delay_timer;
            }
            OpCode::GetKeyPress(x) => {
                // blocks until any key is pressed, then stores that key in rX
            }
            OpCode::SetDelayTimer(x) => {
                // sets delay timer to value of rX
                self.delay_timer = self.registers[x as usize];
            }
            OpCode::SetSoundTimer(x) => {
                // sets sound timer to value of rX
                self.sound_timer = self.registers[x as usize];
            }
            OpCode::AddIndex(x) => {
                // adds rX to index register
                self.index += self.registers[x as usize] as u16;
            }
            OpCode::SetIndexCharacter(x) => {}
            OpCode::StoreBCD(x) => {}
            OpCode::RegDump(x) => {}
            OpCode::RegLoad(x) => {}
            OpCode::NoOp => {}
        };
    }
}
use crate::disassembler;
use crate::disassembler::OpCode;
use crate::display;
use crate::keyboard;
use crate::speaker;

use std::time::{SystemTime, UNIX_EPOCH};

const PROGRAM_OFFSET: usize = 512;
const CLOCK_FREQUENCY_HZ: u32 = 500;
const TIMER_FREQUENCY_HZ: u32 = 60;

pub const FONT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // "0"
    0x20, 0x60, 0x20, 0x20, 0x70, // "1"
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // "2"
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // "3"
    0x90, 0x90, 0xF0, 0x10, 0x10, // "4"
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // "5"
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // "6"
    0xF0, 0x10, 0x20, 0x40, 0x40, // "7"
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // "8"
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // "9"
    0xF0, 0x90, 0xF0, 0x90, 0x90, // "A"
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // "B"
    0xF0, 0x80, 0x80, 0x80, 0xF0, // "C"
    0xE0, 0x90, 0x90, 0x90, 0xE0, // "D"
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // "E"
    0xF0, 0x80, 0xF0, 0x80, 0x80, // "F"
];

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
    next_timer_tick: u128,
    display: display::DisplayBuffer,
    keyboard: keyboard::KeyboardInput,
    speaker: speaker::Speaker
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
            next_timer_tick: 0,
            display: display::DisplayBuffer::new(),
            keyboard: keyboard::KeyboardInput::new(),
            speaker: speaker::Speaker::new()
        }
    }

    pub fn init(&mut self) {
        // load font data into memory
        for i in 0..FONT_SPRITES.len() {
            self.memory[i] = FONT_SPRITES[i];
        }
    }

    pub fn load_program(&mut self, buffer: &[u8]) {
        // load program into memory starting at the program offset
        for i in 0..buffer.len() {
            self.memory[PROGRAM_OFFSET + i] = buffer[i];
        }
    }

    pub fn get_display(&self) -> &display::DisplayBuffer {
        &self.display
    }

    pub fn get_keyboard(&mut self) -> &mut keyboard::KeyboardInput {
        &mut self.keyboard
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

        if self.sound_timer > 0 {
            self.speaker.start();
        } else {
            self.speaker.stop();
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
        match disassembler::disassemble_word(word) {
            Some(opcode) => {
                println!("tick @ 0x{:x?} ({:x?}): {:?}", addr, word, opcode);

                self.advance();
                self.execute_opcode(opcode);
            },
            None => {
                // do nothing
            }
        }
    }

    /// advance to the next instruction
    fn advance(&mut self) {
        self.program_counter += 2;
    }

    fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ClearDisplay => {
                // clears entire display
                self.display.clear();
            }
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
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(val);
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
                let result = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;

                self.registers[x as usize] = (result & 0xFF) as u8;
                self.registers[0xF] = (result > 0xFF) as u8;
            }
            OpCode::Subtract(x, y) => {
                // subtracts rY from rX and sets flag to 0 if there is a borrow
                let result = self.registers[x as usize] as i16 - self.registers[y as usize] as i16;

                self.registers[x as usize] = (result % 0x100i16) as u8;
                self.registers[0xF] = (result >= 0) as u8;
            }
            OpCode::ShiftRight(x) => {
                // stores LSB as flag, then shifts rX to the right once
                let val = self.registers[x as usize];

                self.registers[x as usize] = val >> 1;
                self.registers[0xF] = val & 1;
            }
            OpCode::Difference(x, y) => {
                // sets rX to rY minus rX and sets flag to 0 if there is a borrow
                let result = self.registers[y as usize] as i16 - self.registers[x as usize] as i16;

                self.registers[x as usize] = (result % 0x100i16) as u8;
                self.registers[0xF] = (result >= 0) as u8;
            }
            OpCode::ShiftLeft(x) => {
                // stores MSB as flag, then shifts rX to the left once
                let val = self.registers[x as usize];

                self.registers[x as usize] = val << 1;
                self.registers[0xF] = ((val & 0b1000_0000) > 0) as u8;
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
            OpCode::Rand(x, val) => {
                // set rX to result of bitwise AND of value and random 8-bit integer
                let rand: u8 = rand::random();

                self.registers[x as usize] = rand & val;
            }
            OpCode::DrawSprite(x, y, num_bytes) => {
                // draws a sprite from memory onto the display, and sets collision flag
                let start = self.index as usize;
                let end = start + num_bytes as usize;
                let bytes = &self.memory[start..end];
                let collision = self.display.draw_sprite(self.registers[x as usize], self.registers[y as usize], bytes);

                self.registers[0xF] = collision as u8;
            }
            OpCode::SkipKeyPressed(x) => {
                // skips next instruction if key at rX is pressed
                let key = self.registers[x as usize];

                if self.keyboard.is_key_pressed(key) {
                    self.advance();
                }
            }
            OpCode::SkipKeyNotPressed(x) => {
                // skips next instruction if key at rX is not pressed
                let key = self.registers[x as usize];

                if !self.keyboard.is_key_pressed(key) {
                    self.advance();
                }
            }
            OpCode::GetDelayTimer(x) => {
                // sets rX to value of delay timer
                self.registers[x as usize] = self.delay_timer;
            }
            OpCode::GetKeyPress(x) => {
                // blocks until any key is pressed, then stores that key in rX

                // if we're not already tracking a key, start
                if !self.keyboard.is_tracking_next_key_release() {
                    self.keyboard.track_next_key_release(true);
                }

                // see if a key has been released yet
                if let Some(key) = self.keyboard.get_last_released_key() {
                    self.keyboard.track_next_key_release(false);
                    self.registers[x as usize] = key;
                } else {
                    // no key released, so run this instruction again
                    self.program_counter -= 2;
                }
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
            OpCode::SetIndexCharacter(x) => {
                // sets memory index to sprite of the character that is in a register
                self.index = self.registers[x as usize] as u16 * 5;
            }
            OpCode::StoreBCD(x) => {
                // store binary-coded decimal in memory
                let val = self.registers[x as usize];
                let addr = self.index as usize;

                self.memory[addr] = val / 100;
                self.memory[addr + 1] = (val / 10) % 10;
                self.memory[addr + 2] = val % 10;
            }
            OpCode::RegDump(x) => {
                // stores registers r0 - rX into memory at current index
                for i in 0..=x {
                    self.memory[self.index as usize + i as usize] = self.registers[i as usize];
                }
            }
            OpCode::RegLoad(x) => {
                // reads memory at current index and stores bytes into registers r0 - rX
                for i in 0..=x {
                    self.registers[i as usize] = self.memory[self.index as usize + i as usize];
                }
            }
        };
    }
}
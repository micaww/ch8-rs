# ch8-rs

A basic CHIP-8 interpreter written in Rust.

Features:

- All standard CHIP-8 instructions implemented
- Sound support (via [rodio](https://github.com/RustAudio/rodio))
- Hardware accelerated rendering / scaling (via [pixels](https://github.com/parasyte/pixels))

## Running It

You can run the emulator by either building the source yourself, or choosing a pre-built binary
for your platform in [Releases](https://github.com/micaww/ch8-rs/releases).

#### Building from Source

To build from source, simply run:

`cargo build --release`

### Using the Binary

To use the binary, simply type in the file path to the ROM you want to play as the first argument to the executable.

You can find CHIP-8 ROMs all over the internet. Here is a good source: https://github.com/badlogic/chip8/tree/master/roms

`ch8-rs.exe pong.rom`

### Keyboard Input

CHIP-8 uses a 16-key input with 0-9 buttons as well as A-F.

These keys are mapped to PC keyboards as follows:

```text
|1|2|3|C| => |1|2|3|4|
|4|5|6|D| => |Q|W|E|R|
|7|8|9|E| => |A|S|D|F|
|A|0|B|F| => |Z|X|C|V|
```

Different ROMs use different keys for different things. You might have to
experiment with the keys to figure out how to use each ROM.
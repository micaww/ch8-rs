use winit::event::VirtualKeyCode;

pub struct KeyboardInput {
    key_states: [bool; 16],
    last_released_key: Option<u8>,
    track_next_released_key: bool
}

impl KeyboardInput {
    pub fn new() -> Self {
        KeyboardInput {
            key_states: [false; 16],
            last_released_key: None,
            track_next_released_key: false
        }
    }

    /// Checks if we're tracking the next key release
    pub fn is_tracking_next_key_release(&self) -> bool {
        self.track_next_released_key
    }

    /// Tracks for the next key
    pub fn track_next_key_release(&mut self, track: bool) {
        self.last_released_key = None;
        self.track_next_released_key = track;
    }

    /// Gets the last released key if we're waiting for it
    pub fn get_last_released_key(&self) -> Option<u8> {
        self.last_released_key
    }

    /// Sets a key state
    pub fn set_key_pressed(&mut self, key: u8, pressed: bool) {
        self.key_states[key as usize] = pressed;

        if !pressed && self.track_next_released_key {
            self.last_released_key = Some(key);
        }
    }

    /// Checks if a key is currently pressed
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.key_states[key as usize]
    }
}

/// Gets a virtual key code from a CHIP-8 key
pub fn get_keycode_from_key(key: u8) -> Option<VirtualKeyCode> {
    // CHIP-8 keyboard is mapped to PC as follows:

    // |1|2|3|C| => |1|2|3|4|
    // |4|5|6|D| => |Q|W|E|R|
    // |7|8|9|E| => |A|S|D|F|
    // |A|0|B|F| => |Z|X|C|V|

    match key {
        0x0 => Some(VirtualKeyCode::X),
        0x1 => Some(VirtualKeyCode::Key1),
        0x2 => Some(VirtualKeyCode::Key2),
        0x3 => Some(VirtualKeyCode::Key3),
        0x4 => Some(VirtualKeyCode::Q),
        0x5 => Some(VirtualKeyCode::W),
        0x6 => Some(VirtualKeyCode::E),
        0x7 => Some(VirtualKeyCode::A),
        0x8 => Some(VirtualKeyCode::S),
        0x9 => Some(VirtualKeyCode::D),
        0xA => Some(VirtualKeyCode::Z),
        0xB => Some(VirtualKeyCode::C),
        0xC => Some(VirtualKeyCode::Key4),
        0xD => Some(VirtualKeyCode::R),
        0xE => Some(VirtualKeyCode::F),
        0xF => Some(VirtualKeyCode::V),
        _ => None
    }
}

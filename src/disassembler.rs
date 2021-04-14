// opcode definitions here: https://en.wikipedia.org/wiki/CHIP-8

#[derive(Debug)]
pub enum OpCode {
    ClearDisplay, // 00E0
    Return, // 00EE
    Jump(u16), // 1NNN
    Call(u16), // 2NNN
    SkipEqVal(u8, u8), // 3XNN
    SkipNotEqVal(u8, u8), // 4XNN
    SkipEq(u8, u8), // 5XY0
    SetVal(u8, u8), // 6XNN
    AddVal(u8, u8), // 7XNN
    Copy(u8, u8), // 8XY0
    Or(u8, u8), // 8XY1
    And(u8, u8), // 8XY2
    Xor(u8, u8), // 8XY3
    Add(u8, u8), // 8XY4
    Subtract(u8, u8), // 8XY5
    ShiftRight(u8), // 8XY6
    Difference(u8, u8), // 8XY7
    ShiftLeft(u8), // 8XYE
    SkipNotEq(u8, u8), // 9XY0
    SetIndex(u16), // ANNN
    JumpOffset(u16), // BNNN
    Rand(u8, u8), // CXNN
    DrawSprite(u8, u8, u8), // DXYN
    SkipKeyPressed(u8), // EX9E
    SkipKeyNotPressed(u8), // EXA1
    GetDelayTimer(u8), // FX07
    GetKeyPress(u8), // FX0A
    SetDelayTimer(u8), // FX15
    SetSoundTimer(u8), // FX18
    AddIndex(u8), // FX1E
    SetIndexCharacter(u8), // FX29
    StoreBCD(u8), // FX33
    RegDump(u8), // FX55
    RegLoad(u8) // FX65
}

pub fn disassemble_bytes(bytes: &[u8]) -> Vec<Option<OpCode>> {
    Vec::from(bytes)
        .chunks_exact(2)
        .into_iter()
        .map(|word| disassemble_word(u16::from_be_bytes([word[0], word[1]])))
        .collect()
}

pub fn disassemble_word(word: u16) -> Option<OpCode> {
    let op_1 = ((word & 0xF000) >> 12) as u8;
    let op_2: u8 = ((word & 0x0F00) >> 8) as u8;
    let op_3: u8 = ((word & 0x00F0) >> 4) as u8;
    let op_4: u8 = (word & 0x000F) as u8;

    let x = op_2;
    let y = op_3;
    let nnn = word & 0x0FFF;
    let nn = (word & 0x00FF) as u8;
    let n = op_4;

    match (op_1, op_2, op_3, op_4) {
        (0x0, 0x0, 0xE, 0x0) => Some(OpCode::ClearDisplay),
        (0x0, 0x0, 0xE, 0xE) => Some(OpCode::Return),
        (0x1, _, _, _) => Some(OpCode::Jump(nnn)),
        (0x2, _, _, _) => Some(OpCode::Call(nnn)),
        (0x3, _, _, _) => Some(OpCode::SkipEqVal(x, nn)),
        (0x4, _, _, _) => Some(OpCode::SkipNotEqVal(x, nn)),
        (0x5, _, _, 0x0) => Some(OpCode::SkipEq(x, y)),
        (0x6, _, _, _) => Some(OpCode::SetVal(x, nn)),
        (0x7, _, _, _) => Some(OpCode::AddVal(x, nn)),
        (0x8, _, _, 0x0) => Some(OpCode::Copy(x, y)),
        (0x8, _, _, 0x1) => Some(OpCode::Or(x, y)),
        (0x8, _, _, 0x2) => Some(OpCode::And(x, y)),
        (0x8, _, _, 0x3) => Some(OpCode::Xor(x, y)),
        (0x8, _, _, 0x4) => Some(OpCode::Add(x, y)),
        (0x8, _, _, 0x5) => Some(OpCode::Subtract(x, y)),
        (0x8, _, _, 0x6) => Some(OpCode::ShiftRight(x)),
        (0x8, _, _, 0x7) => Some(OpCode::Difference(x, y)),
        (0x8, _, _, 0xE) => Some(OpCode::ShiftLeft(x)),
        (0x9, _, _, 0x0) => Some(OpCode::SkipNotEq(x, y)),
        (0xA, _, _, _) => Some(OpCode::SetIndex(nnn)),
        (0xB, _, _, _) => Some(OpCode::JumpOffset(nnn)),
        (0xC, _, _, _) => Some(OpCode::Rand(x, nn)),
        (0xD, _, _, _) => Some(OpCode::DrawSprite(x, y, n)),
        (0xE, _, 0x9, 0xE) => Some(OpCode::SkipKeyPressed(x)),
        (0xE, _, 0xA, 0x1) => Some(OpCode::SkipKeyNotPressed(x)),
        (0xF, _, 0x0, 0x7) => Some(OpCode::GetDelayTimer(x)),
        (0xF, _, 0x0, 0xA) => Some(OpCode::GetKeyPress(x)),
        (0xF, _, 0x1, 0x5) => Some(OpCode::SetDelayTimer(x)),
        (0xF, _, 0x1, 0x8) => Some(OpCode::SetSoundTimer(x)),
        (0xF, _, 0x1, 0xE) => Some(OpCode::AddIndex(x)),
        (0xF, _, 0x2, 0x9) => Some(OpCode::SetIndexCharacter(x)),
        (0xF, _, 0x3, 0x3) => Some(OpCode::StoreBCD(x)),
        (0xF, _, 0x5, 0x5) => Some(OpCode::RegDump(x)),
        (0xF, _, 0x6, 0x5) => Some(OpCode::RegLoad(x)),
        _ => None
    }
}

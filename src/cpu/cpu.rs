use std::default;


use crate::memory::memory::{Byte, Memory, Word};
use twelve_bit::u12;


#[allow(non_snake_case)]
pub struct Chip8CPU {

    // Memory
    RAM: Memory,

    // 16 8-bit registers
    VO: Byte,
    V1: Byte,
    V2: Byte,
    V3: Byte,
    V4: Byte,
    V5: Byte,
    V6: Byte,
    V7: Byte,
    V8: Byte,
    V9: Byte,
    VA: Byte,
    VB: Byte,
    VC: Byte,
    VD: Byte,
    VE: Byte,

    // Carry flag // flag register
    VF: Byte, 

    AddressRegister: u12::U12,
    I: Word,
    Stack: Word,

}

impl Default for Chip8CPU {
    fn default() -> Chip8CPU {
        Chip8CPU {
            RAM: Memory::default(),
            VO: 0x00,
            V1: 0x00,
            V2: 0x00,
            V3: 0x00,
            V4: 0x00,
            V5: 0x00,
            V6: 0x00,
            V7: 0x00,
            V8: 0x00,
            V9: 0x00,
            VA: 0x00,
            VB: 0x00,
            VC: 0x00,
            VD: 0x00,
            VE: 0x00,
            VF: 0x00,
            AddressRegister: 0x00.into(),
            I: 0x00,

            Stack: 0x00,
        }
    }
}
pub type Byte = u8;
pub type Word = u16;

pub struct Memory{
    // 4K of memory
    pub buffer: [Byte; 4096],
}

impl Default for Memory{
    fn default() -> Memory{
        Memory{
            buffer: [0; 4096],
        }
    }
}
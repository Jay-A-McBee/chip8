use std::fmt;

pub struct Instruction {
    instruction_bytes: [u8; 2],
    pub first_nibble: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

const BIT_MASK: u8 = 0xF;

impl From<[u8; 2]> for Instruction {
    fn from([hi_byte, lo_byte]: [u8; 2]) -> Self {
        let first_nibble: u8 = hi_byte >> 4 & BIT_MASK;
        let x = hi_byte & BIT_MASK;
        let y = lo_byte >> 4 & BIT_MASK;
        let n = lo_byte & BIT_MASK;
        let nn = lo_byte;
        // actually a 12 bit int
        let nnn = u16::from_be_bytes([x, lo_byte]);

        Instruction {
            instruction_bytes: [hi_byte, lo_byte],
            first_nibble,
            x,
            y,
            n,
            nn,
            nnn,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "----------------------")?;
        writeln!(f, "     Instruction      ")?;
        writeln!(f, "----------------------")?;
        writeln!(f, "Instruction Bytes: {:?}", self.instruction_bytes)?;
        writeln!(f, "OP: {:x}", self.first_nibble)?;
        writeln!(f, "X: {:x}", self.x)?;
        writeln!(f, "Y: {:x}", self.y)?;
        writeln!(f, "N: {:x}", self.n)?;
        writeln!(f, "NN: {:x}", self.nn)?;
        writeln!(f, "NNN: {:x}", self.nnn)?;

        Ok(())
    }
}

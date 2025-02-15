
struct FileHeader {
    seq_type: u8,
    checksum: u8,
}

impl FileHeader {
    fn to_u8(&self) -> u8 {
        self.seq_type << 7 | self.checksum  
    }
}

struct SequenceHeader {
    desc_len: [u8; 2],
    desc: String,
}

enum NAcidMin {
    A,
    C,
    G,
    TU,
}

impl NAcidMin {
    fn from_char(c: char) -> Result<Self, char> {
        match c.to_ascii_lowercase() {
            'a' => Ok(Self::A),
            'c' => Ok(Self::C),
            'g' => Ok(Self::G),
            't' | 'u' => Ok(Self::TU),
            // Padded chars are set as 0b10
            '^' => Ok(Self::G),
            _ => Err(c),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Self::A => 0b00,
            Self::C => 0b01,
            Self::G => 0b10,
            Self::TU => 0b11,
        } 
    }
}

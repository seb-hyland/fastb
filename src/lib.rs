#![allow(dead_code)]

mod sequences;
mod sequence_items;
mod buffer;


static CHECKSUM: u8 = 0b101010;

struct FileHeader {
    seq_type: u8,
}

impl FileHeader {
    fn to_u8(&self) -> u8 {
        (self.seq_type << 6) | CHECKSUM  
    }
}



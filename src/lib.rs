mod sequences;

struct FileHeader {
    seq_type: u8,
    checksum: u8,
}

impl FileHeader {
    fn to_u8(&self) -> u8 {
        (self.seq_type << 7) | self.checksum  
    }
}

struct SequenceHeader {
    desc_len: [u8; 2],
    desc: String,
}


use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Entry {
    desc: String,
    seq: Sequence,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BinaryEntry {
    desc_len: u16,
    seq_len: u32,
    desc: Vec<u8>,
    seq: Vec<u8>,
}

impl BinaryEntry {
    pub fn into_bytes(self) -> Vec<u8> {
        let Self {
            desc_len,
            seq_len,
            desc,
            seq,
        } = self;

        let vector_length = 2 + 4 + desc.len() + seq.len();
        let mut result: Vec<u8> = Vec::with_capacity(vector_length);

        result.extend_from_slice(&desc_len.to_be_bytes());
        result.extend_from_slice(&seq_len.to_be_bytes());
        result.extend_from_slice(&desc);
        result.extend_from_slice(&seq);

        result
    }
}

macro_rules! decode_entry {
    ($self:expr, $type:ident, $desc:expr) => {{
        let width = $type::width();
        let mut bit_buffer = BitReader::new(&$self.seq);
        let mut seq = Vec::new();

        for _ in 0..$self.seq_len {
            let bits = bit_buffer.read(width);
            let seq_item = $type::try_from(bits).map_err(|e| e.to_string())?;
            seq.push(seq_item);
        }

        seq
    }};
}

macro_rules! encode_binary {
    ($self:expr, $type:ident, $desc:expr, $seq:expr) => {{
        let desc_len: u16 = $desc
            .len()
            .try_into()
            .map_err(|e: ConversionError| e.to_string())?;

        let seq_len: u32 = $seq
            .len()
            .try_into()
            .map_err(|e: ConversionError| e.to_string())?;

        let desc = $desc.into_bytes();

        let seq = {
            let width = $type::width();
            let seq_len = $seq.len();

            let buffer_size = if width * seq_len % 8 == 0 {
                width * seq_len / 8
            } else {
                width * seq_len / 8 + 1
            };

            let buffer: Vec<u8> = vec![0; buffer_size];
            let mut buffer = buffer.into_boxed_slice();
            let mut bit_buffer = BitWriter::new(buffer.as_mut());
            if let Sequence::$type(vec) = $seq {
                vec.iter()
                    .copied()
                    .map(|s: $type| -> u8 { s.into() })
                    .for_each(|b: u8| bit_buffer.write(b, width));
                buffer.to_vec()
            } else {
                panic!("Macro expansion resulted in incorrect sequence type");
            }
        };

        Ok(BinaryEntry {
            desc_len,
            seq_len,
            desc,
            seq,
        })
    }};
}

impl BinaryEntry {
    fn into_entry(self, entry_type: Sequence) -> Result<Entry, String> {
        let desc: String = String::from_utf8(self.desc).map_err(|e| e.to_string())?;
        match entry_type {
            Sequence::NucleicAcid(_) => {
                let seq = decode_entry!(self, NucleicAcid, desc);
                Ok(Entry {
                    desc,
                    seq: Sequence::from_NA(seq),
                })
            }
            Sequence::NucleicAcidEx(_) => {
                let seq = decode_entry!(self, NucleicAcidEx, desc);
                Ok(Entry {
                    desc,
                    seq: Sequence::from_NX(seq),
                })
            }
            Sequence::AminoAcid(_) => {
                let seq = decode_entry!(self, AminoAcid, desc);
                Ok(Entry {
                    desc,
                    seq: Sequence::from_AA(seq),
                })
            }
        }
    }
}

impl TryInto<BinaryEntry> for Entry {
    type Error = String;
    fn try_into(self) -> Result<BinaryEntry, Self::Error> {
        type ConversionError = std::num::TryFromIntError;
        match self.seq {
            Sequence::NucleicAcid(_) => encode_binary!(self, NucleicAcid, self.desc, self.seq),
            Sequence::NucleicAcidEx(_) => encode_binary!(self, NucleicAcidEx, self.desc, self.seq),
            Sequence::AminoAcid(_) => encode_binary!(self, AminoAcid, self.desc, self.seq),
        }
    }
}

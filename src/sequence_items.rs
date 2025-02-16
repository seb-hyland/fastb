use crate::{
    buffer::BitBuffer, sequences::{NucleicAcid, Sequence}
};

#[derive(Clone, Debug, PartialEq)]
struct SeqItem<S: Sequence> {
    desc: String,
    seq: Vec<S>
}

#[derive(Clone, Debug, PartialEq)]
struct BinarySeqItem {
    desc_len: u16,
    seq_len: u32,
    desc: Vec<u8>,
    seq: Vec<u8>
}


impl<S> TryInto<SeqItem<S>> for BinarySeqItem where S: Sequence {
    type Error = String;
    
    fn try_into(mut self) -> Result<SeqItem<S>, Self::Error> {
        let desc: String = String::from_utf8(self.desc).map_err(|e| e.to_string())?;
        let seq = {
            let mut result = Vec::new();
            let width = S::bit_width();
            let mut bit_buffer = BitBuffer::new(&mut self.seq);

            for _ in 0..self.seq_len {
                let bits = bit_buffer.read_bits(width);
                let seq_item = S::try_from(bits).map_err(|e| e.to_string())?;
                result.push(seq_item);
            }

            result
        };
        Ok(SeqItem { desc, seq })
    }
}


impl<S> TryInto<BinarySeqItem> for SeqItem<S> where S: Sequence {
    type Error = String;

    fn try_into(self) -> Result<BinarySeqItem, Self::Error> {
        type ConversionError = std::num::TryFromIntError;

        let desc_len: u16 = self.desc.len()
            .try_into()
            .map_err(|e: ConversionError| e.to_string())?;
        let seq_len: u32 = self.seq.len()
            .try_into()
            .map_err(|e: ConversionError| e.to_string())?;
        let desc = self.desc.into_bytes();

        let seq = {
            let width = S::bit_width();
            let seq_len = self.seq.len();
            let buffer_size = if width * seq_len % 8 == 0 {
                width * seq_len / 8
            } else {
                width * seq_len / 8 + 1
            };

            let mut buffer: Vec<u8> = vec![0; buffer_size];
            let mut bit_buffer = BitBuffer::new(&mut buffer);
            self.seq.iter()
                .copied()
                .map(|s: S| -> u8 { s.into() })
                .for_each(|b: u8| bit_buffer.write_bits(b, S::bit_width()));
            buffer
        };

        Ok(BinarySeqItem { desc_len, seq_len, desc, seq })
    }
}

pub fn run() {
    let item = SeqItem { desc: "Hello World!".to_string(), seq: vec![NucleicAcid::ThymineUracil, NucleicAcid::Adenine, NucleicAcid::Guanine, NucleicAcid::Cytosine] };
    let bits: BinarySeqItem = item.clone().try_into().unwrap();
    let undo_item: SeqItem<NucleicAcid> = bits.try_into().unwrap();
    println!("{:?}", undo_item == item);
}

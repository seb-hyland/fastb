use crate::{
    prelude::{AminoAcid, NucleicAcid, NucleicAcidEx},
    sequences::SequenceType,
};

pub(crate) struct BitWriter<'a> {
    buffer: &'a mut [u8],
    cursor: usize,
}

pub(crate) struct BitReader<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

trait BitCursor {
    fn cursor(&self) -> usize;
    fn current_indices(&self) -> (usize, usize) {
        let byte_index = self.cursor() / 8;
        let bit_index = self.cursor() % 8;
        (byte_index, bit_index)
    }
}

impl<'a> BitCursor for BitWriter<'a> {
    fn cursor(&self) -> usize {
        self.cursor
    }
}

impl<'a> BitCursor for BitReader<'a> {
    fn cursor(&self) -> usize {
        self.cursor
    }
}

impl<'a> BitWriter<'a> {
    pub(crate) fn new(buffer: &'a mut [u8]) -> Self {
        BitWriter { buffer, cursor: 0 }
    }

    pub(crate) fn write(&mut self, value: u8, num_bits: usize) {
        assert!(num_bits <= 8, "Attempted to write more than 8 bits at once");

        for i in 0..num_bits {
            let shifted_byte = value >> (num_bits - 1 - i);
            let bit = shifted_byte & 1;

            let (byte_idx, bit_idx) = self.current_indices();
            assert!(byte_idx < self.buffer.len(), "Buffer overflow on write");

            self.buffer[byte_idx] |= bit << (7 - bit_idx);
            self.cursor += 1;
        }
    }
}

impl<'a> BitReader<'a> {
    pub(crate) fn new(buffer: &'a [u8]) -> Self {
        BitReader { buffer, cursor: 0 }
    }

    pub(crate) fn read(&mut self, num_bits: usize) -> u8 {
        assert!(num_bits <= 8, "Cannot read more than 8 bits at a time");

        let mut result = 0;
        for i in 0..num_bits {
            let (byte_idx, bit_idx) = self.current_indices();
            assert!(byte_idx < self.buffer.len(), "Buffer underflow on read");

            let bit = (self.buffer[byte_idx] >> (7 - bit_idx)) & 1;
            result |= bit << (num_bits - 1 - i);
            self.cursor += 1;
        }
        result
    }

    pub(crate) fn read_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        let (byte_idx, bit_idx) = self.current_indices();
        assert!(bit_idx == 0, "Attempted to read bytes with invalid cursor");

        let mut result = Vec::new();
        (0..num_bytes).for_each(|i| {
            result.push(self.buffer[byte_idx + i]);
            self.cursor += 8;
        });

        assert!(
            result.len() == num_bytes,
            "Attempted to return invalid length of bytes"
        );
        result
    }

    pub(crate) fn read_NA(&mut self) -> Result<NucleicAcid, u8> {
        self.read(SequenceType::NucleicAcid.width()).try_into()
    }

    pub(crate) fn read_NX(&mut self) -> Result<NucleicAcidEx, u8> {
        self.read(SequenceType::NucleicAcidEx.width()).try_into()
    }

    pub(crate) fn read_AA(&mut self) -> Result<AminoAcid, u8> {
        self.read(SequenceType::AminoAcid.width()).try_into()
    }

    pub(crate) fn at_end(&self) -> bool {
        self.cursor / 8 == self.buffer.len() - 1
    }
}

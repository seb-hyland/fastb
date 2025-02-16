pub(crate) struct BitBuffer<'a> {
    buffer: &'a mut [u8],
    cursor: usize,
}

impl<'a> BitBuffer<'a> {
    pub(crate) fn new(buffer: &'a mut Vec<u8>) -> Self {
        BitBuffer {
            buffer,
            cursor: 0,
        }
    }

    fn current_indices(&self) -> (usize, usize) {
        let byte_index = self.cursor / 8;
        let bit_index = self.cursor % 8;
        (byte_index, bit_index)
    }

    pub(crate) fn write_bits(&mut self, value: u8, num_bits: usize) {
        assert!(num_bits <= 8, "Attempted to write more than 8 bits at once");

        for i in 0..num_bits {
            let shifted_byte = value >> (num_bits - 1 - i);
            let bit = shifted_byte & 1;

            let (byte_idx, bit_idx) = self.current_indices();
            assert!(byte_idx <= self.buffer.len(), "Buffer overflow on write");

            self.buffer[byte_idx] |= bit << (7 - bit_idx);
            self.cursor += 1;
        }
    }

    pub(crate) fn read_bits(&mut self, num_bits: usize) -> u8 {
        assert!(num_bits <= 8, "Cannot read more than 8 bits at a time");

        let mut result = 0;
        for i in 0..num_bits {
            let (byte_idx, bit_idx) = self.current_indices();
            assert!(byte_idx <= self.buffer.len(), "Buffer underflow on read");

            let bit = (self.buffer[byte_idx] >> (7 - bit_idx)) & 1;
            result |= bit << (num_bits - 1 - i);
            self.cursor += 1;
        }
        result
    }
}

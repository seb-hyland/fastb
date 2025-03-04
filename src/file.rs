use std::{
    fs::{read, write},
    path::PathBuf,
};

#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelRefIterator;

use crate::prelude::*;

static CHECKSUM: u8 = 0b101010;
type Job = (Box<[u8]>, Box<[u8]>);

pub struct File {
    header: Sequence,
    contents: Vec<Entry>,
}

impl File {
    fn new(header: Sequence) -> Self {
        let contents = Vec::new();
        Self { header, contents }
    }

    fn write_fastb(self, path: PathBuf) -> Result<(), String> {
        let output: Vec<u8> = self.try_into()?;
        write(path, output).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn new_from_fastb(file: &[u8]) -> Result<Self, String> {
        let mut bit_buffer = BitReader::new(file);
        let bits = bit_buffer.read(2);

        let checksum = bit_buffer.read(6);
        if checksum != CHECKSUM {
            Err("Invalid checksum!".to_string())
        } else {
            match bits {
                0b00 => Ok(Self::new(Sequence::new_NA())),
                0b01 => Ok(Self::new(Sequence::new_NX())),
                0b10 => Ok(Self::new(Sequence::new_AA())),
                _ => Err("Invalid sequence type identifier".to_string()),
            }
        }
    }

    fn extract_jobs(file: &[u8]) -> Result<Vec<Job>, String> {
        let mut bit_buffer = BitReader::new(file);
        let _ = bit_buffer.read_bytes(1);

        let mut jobs: Vec<Job> = Vec::new();
        for _ in 0..u32::MAX {
            if bit_buffer.at_end() {
                break;
            }

            let desc_len = bit_buffer.read_bytes(2);
            let desc_len = u16::from_be_bytes(desc_len.try_into().expect("Expected 2 bytes."));

            let seq_len = bit_buffer.read_bytes(4);
            let seq_len = u32::from_be_bytes(seq_len.try_into().expect("Expected 4 bytes."));

            let desc = bit_buffer.read_bytes(desc_len.into());
            let seq = bit_buffer.read_bytes(
                seq_len
                    .try_into()
                    .expect("Platform pointer is smaller than 32 bits."),
            );

            jobs.push((desc.into_boxed_slice(), seq.into_boxed_slice()));
        }

        Ok(jobs)
    }

    fn process_jobs(jobs: Vec<Job>, width: usize) -> Vec<Entry> {
        let _ = jobs.iter().map(|(desc, contents)| {
            let description = String::from_utf8_lossy(desc).into_owned();
            let data = {
                let mut bit_buffer = BitReader::new(contents);
                let reps = (contents.len() * 8) / width;
                for _ in 0..reps {
                    let desc_len = bit_buffer.read_bytes(2);
                    let desc_len = u16::from_be_bytes(desc_len.try_into()
                        .expect("Expected 2 bytes."));

                    let seq_len = bit_buffer.read_bytes(4);
                    let seq_len =
                        u32::from_be_bytes(seq_len.try_into().expect("Expected 4 bytes."));

                    let desc = bit_buffer.read_bytes(desc_len.into());
                    let seq = bit_buffer.read_bytes(
                        seq_len
                            .try_into()
                            .expect("Platform pointer is smaller than 32 bits."),
                    );
                }
            };
        });
        todo!()
    }

    #[cfg(feature = "rayon")]
    fn process_jobs_parallel(jobs: Vec<Job>) -> Vec<Entry<S>> {
        todo!()
    }
}

impl TryInto<Vec<u8>> for File {
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let Self { header, contents } = self;

        let mut result = Vec::new();
        let seq_id: u8 = match header {
            SequenceType::NucleicAcid => 0b00,
            SequenceType::NucleicAcidEx => 0b01,
            SequenceType::AminoAcid => 0b10,
        };
        let header = (seq_id << 6) | CHECKSUM;
        result.push(header);

        let binary_contents: Vec<BinaryEntry> = contents
            .iter()
            .cloned()
            .map(|s: Entry| s.try_into())
            .collect::<Result<Vec<BinaryEntry>, _>>()?;

        binary_contents.iter().cloned().for_each(|i| {
            result.extend_from_slice(&i.into_bytes());
        });

        Ok(result)
    }
}

// fn test(id: i32) -> Vec<Entry<Box<dyn Sequence>>> {
//     if id < 5 {
//         let vec: Vec<Box<dyn Sequence>> = vec![
//             Box::new(NucleicAcid::ThymineUracil),
//             Box::new(NucleicAcid::Adenine),
//         ];
//         let entry = Entry::new("Hello".to_string(), vec);
//         vec![entry]
//     } else {
//         let vec: Vec<Box<dyn Sequence>> = vec![
//             Box::new(AminoAcid::Alanine),
//             Box::new(AminoAcid::AsparticAcid),
//         ];
//         let entry = Entry::new("Good".to_string(), vec);
//         vec![entry]
//     }
// }

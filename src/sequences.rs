pub trait Sequence: Into<u8> + TryFrom<u8, Error = u8>
where
    Self: Sized + Copy,
{
    fn from_char(c: char) -> Result<Self, char>;
    fn bit_width() -> usize;
}

macro_rules! define_sequence_type {
    ($name:ident[$width:expr] { $($variant:ident : $( $char:literal ),+ => $value:expr ),* }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub(crate) enum $name {
            $( $variant, )*
        }

        impl Sequence for $name {
            fn from_char(c: char) -> Result<Self, char> {
                match c.to_ascii_lowercase() {
                    $( $( $char => Ok(Self::$variant), )+ )*
                    _ => Err(c),
                }
            }

            fn bit_width() -> usize {
                $width
            }
        }


        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value, )*
                }
            }
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = u8;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $( $value => Ok(Self::$variant), )*
                    _ => Err(value),
                }
            }
        }
    };
}

define_sequence_type!(NucleicAcid[2] {
    Adenine : 'a' => 0b00,
    Cytosine : 'c' => 0b01,
    Guanine : 'g', '^' => 0b10,
    ThymineUracil : 't', 'u' => 0b11
});

define_sequence_type!(NucleicAcidExpanded[4] {
    Adenine : 'a' => 0b0001,
    Cytosine : 'c' => 0b0010,
    Guanine : 'g' => 0b0100,
    ThymineUracil : 't', 'u' => 0b1000,
    Purine : 'r' => 0b0101,
    Pyrimidine : 'y', '^' => 0b1010,
    Ketone : 'k' => 0b1100,
    AminoGroup : 'm' => 0b0011,
    StrongInteraction : 's' => 0b0110,
    WeakInteraction : 'w' => 0b1001,
    NotA : 'b' => 0b1110,
    NotC : 'd' => 0b1101,
    NotG : 'h' => 0b1011,
    NotTU : 'v' => 0b0111,
    NucleicAcid : 'n' => 0b1111,
    Gap : '-' => 0b0000
});

define_sequence_type!(AminoAcid[5] {
    Alanine : 'a' => 0b00000,
    DorN : 'b' => 0b00001,
    Cysteine : 'c' => 0b00010,
    AsparticAcid : 'd' => 0b00011,
    GlutamicAcid : 'e' => 0b00100,
    Phenylalanine : 'f' => 0b00101,
    Glycine : 'g' => 0b00110,
    Histidine : 'h' => 0b00111,
    Isoleucine : 'i' => 0b01000,
    IorL : 'j' => 0b01001,
    Lysine : 'k' => 0b01010,
    Leucine : 'l' => 0b01011,
    Methionine : 'm' => 0b01100,
    Asparagine : 'n' => 0b01101,
    Pyrrolysine : 'o' => 0b01110,
    Proline : 'p' => 0b01111,
    Glutamine : 'q' => 0b10000,
    Arginine : 'r' => 0b10001,
    Serine : 's' => 0b10010,
    Threonine : 't' => 0b10011,
    Selenocysteine : 'u' => 0b10100,
    Valine : 'v', '^' => 0b10101,
    Tryptophan : 'w' => 0b10110,
    Tyrosine : 'y' => 0b10111,
    EorQ : 'z' => 0b11000,
    Any : 'x' => 0b11001,
    Stop : '*' => 0b11010,
    Gap : '-' => 0b11011
});

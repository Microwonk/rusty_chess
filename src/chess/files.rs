use super::bitboards::{BitBoard, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H};

pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn bitboard(&self) -> BitBoard {
        match self {
            File::A => FILE_A,
            File::B => FILE_B,
            File::C => FILE_C,
            File::D => FILE_D,
            File::E => FILE_E,
            File::F => FILE_F,
            File::G => FILE_G,
            File::H => FILE_H,
             
        }
    }
}
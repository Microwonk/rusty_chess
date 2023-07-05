#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Piece {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Bishop,
            2 => Piece::Knight,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => Piece::Pawn,
        }
    }
}

impl Into<usize> for Piece {
    fn into(self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Bishop => 1,
            Piece::Knight => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
}

pub fn index_from_tuple(tuple: &(Piece, PieceColor)) -> usize {
    let mut index: usize = tuple.0.into();
    index += if tuple.1 == PieceColor::White { 6 } else { 0 };
    index
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PieceColor {
    Black,
    White,
}

impl From<usize> for PieceColor {
    fn from(value: usize) -> Self {
        match value {
            1 => PieceColor::White,
            _ => PieceColor::Black,
        }
    }
}

impl Into<usize> for PieceColor {
    fn into(self) -> usize {
        match self {
            PieceColor::Black=> 0,
            PieceColor::White => 1,
        }
    }
}


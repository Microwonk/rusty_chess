#[derive(PartialEq, Eq, Debug, Clone, Copy, Ordinalize)]
pub enum Piece {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub fn index_from_tuple(tuple: &(Piece, PieceColor)) -> usize {
    let mut index: usize = tuple.0.ordinal() as usize;
    index += if tuple.1 == PieceColor::White { 6 } else { 0 };
    index
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PieceColor {
    Black = 0,
    White = 1,
}


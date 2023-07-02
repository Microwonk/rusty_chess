#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PieceColor {
    Black,
    White,
}
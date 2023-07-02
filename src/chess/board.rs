use super::bitboards::{BitBoard, DEFAULT_PAWNS, DEFAULT_BISHOPS, DEFAULT_KNIGHTS, DEFAULT_ROOKS, DEFAULT_QUEENS, DEFAULT_KINGS, DEFAULT_BLACK_MASK, DEFAULT_WHITE_MASK};
use super::consts::{NUM_PIECES, NUM_COLORS};

pub struct Board {
    pub pieces: [BitBoard; NUM_PIECES],
    pub colors: [BitBoard; NUM_COLORS],
    pub combined: BitBoard,
    // move_maker: PieceColor,
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            pieces: [
                DEFAULT_PAWNS,
                DEFAULT_BISHOPS,
                DEFAULT_KNIGHTS,
                DEFAULT_ROOKS,
                DEFAULT_QUEENS,
                DEFAULT_KINGS
            ],
            colors: [DEFAULT_BLACK_MASK, DEFAULT_WHITE_MASK],
            combined: DEFAULT_WHITE_MASK | DEFAULT_BLACK_MASK,
         }
    }
}

use super::{bitboards::{BitBoard, self}, board::Board, squares::Square};

///              before  ,    after
pub type Move = (BitBoard, BitBoard);

pub fn get_legal_moves(board: Board, square: Square) -> BitBoard {
    return bitboards::EMPTY;
}
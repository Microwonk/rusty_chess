use std::ops::{BitOr, BitAnd, BitOrAssign, BitXor, BitAndAssign, BitXorAssign, Not, Mul};

pub const EMPTY: BitBoard = 0;
pub const DEFAULT_PAWNS: BitBoard =  0x000000000000FF00;
pub const DEFAULT_BISHOPS: BitBoard = 0x0000000000000024;
pub const DEFAULT_KNIGHTS: BitBoard = 0x0000000000000042;
pub const DEFAULT_ROOKS: BitBoard = 0x0000000000000081;
pub const DEFAULT_QUEENS: BitBoard = 0x0000000000000008;
pub const DEFAULT_KINGS: BitBoard = 0x0000000000000010;
pub const DEFAULT_WHITE_MASK: BitBoard = 0x000000000000FFFF;
pub const DEFAULT_BLACK_MASK: BitBoard = 0xFFFF000000000000;

pub type BitBoard = u64;
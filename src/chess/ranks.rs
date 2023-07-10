use super::bitboards::{BitBoard, RANK_ONE, RANK_TWO, RANK_THREE, RANK_FOUR, RANK_FIVE, RANK_SIX, RANK_SEVEN, RANK_EIGHT};

#[allow(dead_code)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn bitboard(&self) -> BitBoard {
        match self {
            Rank::One => RANK_ONE,
            Rank::Two => RANK_TWO,
            Rank::Three => RANK_THREE,
            Rank::Four => RANK_FOUR,
            Rank::Five => RANK_FIVE,
            Rank::Six => RANK_SIX,
            Rank::Seven => RANK_SEVEN,
            Rank::Eight => RANK_EIGHT,
        }
    }
}
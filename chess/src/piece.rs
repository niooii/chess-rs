use crate::{piece_rules::MoveRules}

// hardcode for now
pub struct Piece {
    move_rules: Vec<MoveRules>
}

impl Piece {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

pub mod defaults {
    use crate::piece::Piece;

    pub fn pawn() -> Piece {
        Piece::new()
    }
}
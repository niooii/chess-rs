use std::sync::{Arc, RwLock};

use crate::error::Result;
use crate::piece::Piece;

#[derive(Clone, Copy)]
// ALL OFFSETS ARE TO THE LEFT, RELATIVE TO THE ORIENTATION OF THE BOARD.
pub enum StartInfo {
    // ALL OFFSETS ARE TO THE LEFT, RELATIVE TO THE ORIENTATION OF THE BOARD.
    Bottom { offset: u32 },
    Top { offset: u32 },
    Left { offset: u32 },
    Right { offset: u32 },
}

pub struct Team {
    name: String,
    start_info: StartInfo,
}

impl Team {
    pub fn new(name: String, start_direction: StartInfo) -> Self {
        Self {
            name,
            start_info: start_direction,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn start_info(&self) -> StartInfo {
        self.start_info
    }
}

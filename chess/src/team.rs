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
    current_move_number: u32
}

impl Team {
    pub fn new(name: String, start_direction: StartInfo) -> Self {
        Self {
            name,
            start_info: start_direction,
            current_move_number: 0
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn start_info(&self) -> StartInfo {
        self.start_info
    }

    pub fn current_move_number(&self) -> u32 {
        self.current_move_number
    }

    pub fn increment_move(&mut self) {
        self.current_move_number += 1;
    }

    
    pub fn decrement_move(&mut self) {
        self.current_move_number -= 1;
    }
}

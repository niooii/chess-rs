use std::sync::Arc;

use crate::{team::Team, piece::Piece};

// TODO: instead of populatiang manually or whatever, give each piece a relative starting coord and popullate by looping through piecesets.
#[derive(Clone)]
pub struct PieceSet {
    team: Arc<Team>,
    pieces: Vec<Piece>
}

impl PieceSet {
    pub fn new(team: Arc<Team>, pieces: Vec<Piece>) -> Self {
        Self {
            team,
            pieces
        }
    }

    pub fn team(&self) -> Arc<Team> {
        self.team.clone()
    }

    pub fn pieces(&self) -> Vec<Piece> {
        self.pieces.clone()
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.pieces.push(piece)
    }

    pub fn alive_pieces(&self) -> () {
        // self.pieces.iter().filter(|p| p.read().unwrap().is_alive())
        todo!();
    }
}
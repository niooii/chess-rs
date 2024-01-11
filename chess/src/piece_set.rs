use std::sync::Arc;

use crate::error::Result;
use crate::piece::{self, PieceBuilder};
use crate::{piece::Piece, r#move::Coord, team::Team};

// TODO: instead of populatiang manually or whatever, give each piece a relative starting coord and popullate by looping through piecesets.
#[derive(Clone)]
pub struct PieceSet {
    team: Arc<Team>,
    // these two should always be in sync, probably.
    pieces: Vec<Piece>,
    starting_coords: Vec<Coord>,
}

impl PieceSet {
    pub fn new(team: Arc<Team>, pieces: Vec<Piece>) -> Self {
        Self {
            team,
            pieces,
            starting_coords: Vec::new(),
        }
    }

    pub fn team(&self) -> Arc<Team> {
        self.team.clone()
    }

    pub fn pieces(&self) -> Vec<Piece> {
        self.pieces.clone()
    }

    pub fn starting_coords(&self) -> Vec<Coord> {
        self.starting_coords.clone()
    }

    /// automatically adds piece
    /// also remove result return maybe later idk
    pub fn add_piece(&mut self, piece: Piece, rel_starting_coord: Coord) -> Result<()> {
        let mut lock = piece.write().unwrap();
        lock.set_team(self.team.clone());
        lock.set_rel_pos(rel_starting_coord);
        drop(lock);
        self.pieces.push(piece);
        self.starting_coords.push(rel_starting_coord);

        Ok(())
    }

    pub fn alive_pieces(&self) -> () {
        // self.pieces.iter().filter(|p| p.read().unwrap().is_alive())
        todo!();
    }

    // makes a copy of all the data for another team.
    pub fn clone_for_team(&self, team: Arc<Team>) -> Self {

        Self {
            team: team.clone(),
            pieces: {
                self.pieces.iter().map(|p| {
                    let new_p = PieceBuilder::clone_piece(p);
                    new_p.write().unwrap()
                    .set_team(team.clone());
                new_p
                }).collect()
            },
            starting_coords: self.starting_coords.clone()
        }
    }
}

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use thiserror::Error;

use crate::error::{ChessError, Result};
use crate::piece;
use crate::piece::Piece;
use crate::piece::PieceRef;
use crate::team::Team;

pub type Tile = Arc<RwLock<TileRef>>;

#[derive(Clone)]
pub struct TileRef {
    piece: Option<Piece>,
    team_on_tile: Option<Arc<Team>>
}

impl TileRef {
    pub fn piece(&self) -> Option<Piece> {
        self.piece.clone()
    }

    pub fn occupied(&self) -> bool {
        return self.piece.is_some();
    }

    pub fn vacant(&self) -> bool {
        return self.piece.is_none();
    }

    pub fn set_piece(&mut self, piece: Piece) -> Result<()> {
        if self.piece.is_some() {
            let read_lock = self.piece.as_ref().unwrap().read().unwrap();
            return Err(ChessError::TileActionError {
                why: format!(
                    "Could not replace tile's piece, there is already a {} here.",
                    read_lock.name()
                ),
            });
        }
        self.piece = Some(piece.clone());
        self.team_on_tile = piece.read().unwrap().team();
        Ok(())
    }

    pub fn replace_piece(&mut self, piece: Piece) {
        self.piece = Some(piece.clone());
        self.team_on_tile = piece.read().unwrap().team();
    }

    pub fn remove_piece(&mut self) -> Result<()> {
        if self.piece.is_none() {
            return Err(ChessError::TileActionError {
                why: format!("Could not remove tile's piece, it is already vacant."),
            });
        }
        self.piece = None;
        self.team_on_tile = None;
        Ok(())
    }

    pub fn move_contained_piece(&mut self, target_tile: Tile) -> Result<()> {
        if self.piece.is_none() {
            return Err(ChessError::TileActionError {
                why: "Could not move contained piece, there is no piece to move.".to_string(),
            });
        }

        let mut lock = target_tile.write().unwrap();
        lock.set_piece(self.piece.clone().unwrap())?;
        drop(lock);

        self.remove_piece();

        Ok(())
    }

    pub fn team_on_tile(&self) -> Option<Arc<Team>> {
        self.team_on_tile.clone()
    }

    pub fn team_on_tile_unchecked(&self) -> Arc<Team> {
        self.team_on_tile.as_ref().unwrap().clone()
    }
}

#[derive(Default)]
pub struct TileBuilder {
    
}

impl TileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Tile {
        Arc::new(RwLock::new(TileRef {piece: None, team_on_tile: None}))
    }
}

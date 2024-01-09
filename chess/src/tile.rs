use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use thiserror::Error;

use crate::error::{ChessError, Result};
use crate::piece;
use crate::piece::Piece;
use crate::piece::PieceRef;

pub type Tile = Arc<RwLock<TileRef>>;

#[derive(Clone)]
pub struct TileRef {
    piece: Option<Piece>,
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
        self.piece = Some(piece);
        Ok(())
    }

    pub fn replace_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn remove_piece(&mut self) -> Result<()> {
        if self.piece.is_none() {
            return Err(ChessError::TileActionError {
                why: format!("Could not remove tile's piece, it is already vacant."),
            });
        }
        self.piece = None;
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

        self.piece = None;

        Ok(())
    }
}

#[derive(Default)]
pub struct TileBuilder {
    piece: Option<Piece>,
}

impl TileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn piece(mut self, piece: Piece) -> Self {
        self.piece = Some(piece);

        self
    }

    pub fn build(self) -> Tile {
        Arc::new(RwLock::new(TileRef { piece: self.piece }))
    }
}

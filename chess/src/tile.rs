use std::sync::Mutex;
use std::sync::Arc;

use crate::piece::Piece;

pub struct Tile {
     piece: Option<Arc<Mutex<Piece>>>
}

impl Tile {
     /// This WILL crash if there is no piece. 
     fn piece(&self) -> Arc<Mutex<Piece>> {
          self.piece.unwrap().clone()
     }

     fn has_piece(&self) -> bool {
          return self.piece.is_some()
     }

     fn is_empty(&self) -> bool {
          return self.piece.is_none()
     }
}
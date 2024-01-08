use std::sync::Mutex;
use std::sync::Arc;
use std::sync::RwLock;

use crate::piece;
use crate::piece::PieceRef;
use crate::piece::Piece;

pub type Tile = Arc<RwLock<TileRef>>;

#[derive(Clone)]
pub struct TileRef {
     piece: Option<Piece>
}

impl TileRef {
     pub fn piece(&self) -> Option<Piece> {
          self.piece.clone()
     }

     pub fn occupied(&self) -> bool {
          return self.piece.is_some()
     }

     pub fn vacant(&self) -> bool {
          return self.piece.is_none()
     }
}

#[derive(Default)]
pub struct TileBuilder {
     piece: Option<Piece>
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
          Arc::new(
               RwLock::new(
                    TileRef {
                         piece: self.piece
                    }
               )
          )
     }
}
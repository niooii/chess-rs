use crate::{tile::Tile, piece::Piece};
use crate::errors::Result;

#[derive(Clone, Copy)]
pub struct Coord {
    x: u32,
    y: u32
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct Move {
    from: Coord,
    to: Coord,

    piece: Piece,
    from_tile: Tile,
    target_tile: Tile
}

impl Move {
    pub fn new(piece: Piece, from: Coord, to: Coord, from_tile: Tile, target_tile: Tile) -> Self {
        Self {
            from,
            to,
            piece,
            from_tile,
            target_tile
        }
    }

    // accessors
    pub fn from(&self) -> Coord {
        self.from
    }

    pub fn to(&self) -> Coord {
        self.to
    }

    pub fn piece(&self) -> Piece {
        self.piece.clone()
    }

    pub fn from_tile(&self) -> Tile {
        self.from_tile.clone()
    }

    pub fn target_tile(&self) -> Tile {
        self.target_tile.clone()
    }

    // actual stuff
    pub fn execute(self) -> MoveLog {

        

        MoveLog {

        }
    }
}

pub struct MoveLog {
    from: Coord,
    to: Coord,

    moved_piece: Piece,
    killed_piece: Option<Piece>,
    promoted_to: Option<Piece>,

    moved_from: Tile,
    moved_to: Tile
}

impl MoveLog {
    /// undos the logged move.
    /// usually will panick at incorrect board state.
    /// MUST BE EXECUTED IN ORDER.
    pub fn reverse(self) -> Result<()> {
        // undo moved piece's move.
        let mut lock = self.moved_to.write().unwrap();
        lock.move_contained_piece(self.moved_from);
        // if the piece was promoted, replace the new promtoed one with the original oen.
        if self.promoted_to.is_some() {
            lock.replace_piece(self.moved_piece);
        }

        // if there was a piece killed, set killed piece back at its original spot. (also revive it)
        if self.killed_piece.is_some() {
            let piece_ref = self.killed_piece.clone().unwrap();
            let mut kp_lock = piece_ref.write().unwrap();
            kp_lock.revive();
            drop(kp_lock);
            
            lock.set_piece(self.killed_piece.unwrap())?;
        }

        Ok(())
    }
}
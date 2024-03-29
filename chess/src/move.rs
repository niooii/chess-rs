use crate::error::{Result, ChessError};
use crate::team::StartInfo;
use crate::vec2::Vec2;
use crate::{piece::Piece, tile::Tile};

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn add(&self, other: &Coord) -> Self {
        Self {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }

    // TODO! handle overflows later.
    pub fn translate(&self, vec: &Vec2) -> Result<Self> {
        if self.x as i32 + vec.x() < 0 || self.y as i32 + vec.y() < 0 {
            return Err(ChessError::CoordTranslationError { why: "Resultant coordinate has a value under 0.".to_string() });
        }

        Ok(
            Self {
                x: (self.x as i32 + vec.x()) as u32,
                y: (self.y as i32 + vec.y()) as u32
            }
        )
    }
}

pub struct Move {
    from: Coord,
    to: Coord,
    // to translate the piece's relative position when executing the move.
    rel_translation: Coord,

    piece: Piece,
    from_tile: Tile,
    target_tile: Tile,
}

impl Move {
    pub fn new(piece: Piece, from: Coord, to: Coord, rel_translation: Coord, from_tile: Tile, target_tile: Tile) -> Self {
        Self {
            from,
            to,
            rel_translation,
            piece,
            from_tile,
            target_tile,
        }
    }

    // accessors
    pub fn from(&self) -> Coord {
        self.from
    }

    pub fn to(&self) -> Coord {
        self.to
    
    }

    pub fn rel_translation(&self) -> Coord {
        self.rel_translation
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
    pub fn execute(self) -> Result<MoveLog> {
        let mut killed_piece = None;
        let mut target_lock = self.target_tile.write().unwrap();
        if target_lock.occupied() {
            killed_piece = target_lock.piece();

            let mut killed_lock = killed_piece.as_ref().unwrap().write().unwrap();
            killed_lock.kill();
            drop(killed_lock);

            target_lock.remove_piece()?;
        }

        drop(target_lock);

        self.from_tile
            .write()
            .unwrap()
            .move_contained_piece(self.target_tile.clone())?;

        let mut write_lock = self.piece.write().unwrap();
        write_lock.translate_rel_pos(self.rel_translation.x(), self.rel_translation.y());
        drop(write_lock);

        Ok(MoveLog {
            from: self.from,
            to: self.to,

            moved_piece: self.piece,
            killed_piece: killed_piece,
            // TODO: IMPLEMENT LATER!
            promoted_to: None,

            moved_from: self.from_tile,
            moved_to: self.target_tile,
        })
    }
}

pub struct MoveLog {
    from: Coord,
    to: Coord,

    moved_piece: Piece,
    killed_piece: Option<Piece>,
    promoted_to: Option<Piece>,

    moved_from: Tile,
    moved_to: Tile,
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

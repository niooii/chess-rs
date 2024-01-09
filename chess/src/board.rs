// support up to four teams later

use crate::error::Result;
use crate::{
    piece::Piece,
    piece_set::PieceSet,
    r#move::Coord,
    team::StartInfo,
    tile::{self, Tile, TileBuilder},
};

pub struct Board {
    width: u32,
    height: u32,

    tiles: Box<[Box<[Tile]>]>,
    piece_sets: Vec<PieceSet>,
}

impl Board {
    pub fn new(w: u32, h: u32) -> Self {
        let mut tiles = Vec::with_capacity(h as usize);

        for _ in 0..h {
            let mut row = Vec::<Tile>::with_capacity(w as usize);
            for _ in 0..w {
                row.push(TileBuilder::new().build());
            }
            tiles.push(row.into_boxed_slice());
        }

        Self {
            width: w,
            height: h,

            // mistake, assume this constructor exists
            tiles: tiles.into_boxed_slice(),
            piece_sets: Vec::new(),
        }
    }

    pub fn add_piece_set(&mut self, set: PieceSet) -> Result<()> {
        self.piece_sets.push(set);

        Ok(())
    }

    /// should only be called once
    pub fn init(&mut self) -> Result<()> {
        for set in &self.piece_sets {
            let team = set.team().clone();
            let rel_starting_coords = set.starting_coords();

            for (i, piece) in set.pieces().iter().enumerate() {
                let start = rel_starting_coords[i];
                let start_info = team.start_info();
                let abs_coord = self.rel_coord_to_absolute(start, start_info);
                // add the translation vector (offset) from relative to absolute space.
                let tile_opt = match start_info {
                    StartInfo::Bottom { offset } => self.tile_at(abs_coord.x() + offset, abs_coord.y()).unwrap(),
                    StartInfo::Top { offset } => self.tile_at(abs_coord.x() - offset, abs_coord.y()).unwrap(),
                    StartInfo::Left { offset } => self.tile_at(abs_coord.x(), abs_coord.y() - offset).unwrap(),
                    StartInfo::Right { offset } => self.tile_at(abs_coord.x(), abs_coord.y() + offset).unwrap(),
                };
                //

                let mut tile_lock = tile_opt.write().unwrap();
                tile_lock.set_piece(piece.clone())?;
                drop(tile_lock);
            }
        }

        Ok(())
    }

    /// TODO: implement error checking later (out of bounds)
    /// FOLLOWS INDEXING RULES. starts at 0 (where 0 on the y axis is the BOTTOM tile, max height is the TOP one.).
    pub fn tile_at(&self, x: u32, y: u32) -> Option<Tile> {
        if self.height - 1 < y {
            return None;
        }
        let row = self.tiles.get(self.height as usize - 1 - y as usize);
        if let Some(r) = row {
            return r.get(x as usize).cloned();
        }

        None
    }

    pub fn rel_coord_to_absolute(&self, original: Coord, start_info: StartInfo) -> Coord {
        let mut x = original.x();
        let mut y = original.y();

        let abs = match start_info {
            StartInfo::Bottom { .. } => original,
            StartInfo::Left { .. } => Coord::new(y, self.height - 1 - x),
            StartInfo::Right { .. } => Coord::new(self.width - 1 - y, x),
            StartInfo::Top { .. } => Coord::new(self.width - 1 - x, self.height - 1 - y),
        };

        abs
    }

    // for debugging stuff
    pub fn print_state(&self) {
        let mut board_str = String::new();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let tile = self.tile_at(x, y).unwrap();
                let maybe_piece = tile.read().unwrap().piece();

                if let Some(p) = maybe_piece {
                    let first_letter = p.read().unwrap().name().chars().nth(0).unwrap();
                    board_str.push(first_letter);
                } else {
                    board_str.push(' ');
                }
                board_str.push(' ');
            }
            board_str.push('\n');
        }
        println!("{}", board_str);
        // let reversed_str: String = board_str.chars().rev().collect();
        // println!("{reversed_str}");
    }
}

// ooh ooh aah aah
impl Board {
    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn piece_sets(&self) -> Vec<PieceSet> {
        self.piece_sets.clone()
    }
}

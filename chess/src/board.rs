// support up to four teams later

use crate::{tile::{Tile, TileBuilder}, piece::Piece, piece_set::PieceSet};
use crate::error::Result;

pub struct Board {
    width: u32,
    height: u32,
    
    tiles: Box<[Box<[Tile]>]>,
    piece_sets: Vec<PieceSet>
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
            piece_sets: Vec::new()
        }
    }

    pub fn add_piece_set(&mut self, set: PieceSet, tile: Tile) -> Result<()> {
        self.piece_sets.push(set);
        


        Ok(())
    }

    /// TODO: implement error checking later (out of bounds)
    /// FOLLOWS INDEXING RULES. starts at 0.
    pub fn tile_at(&self, x: u32, y: u32) -> Option<Tile> {
        let row = self.tiles.get(y as usize);
        if let Some(r) = row {
            return r.get(x as usize).cloned();
        } 

        None
    }

    // for debugging stuff
    pub fn print_state(&self) {
        let mut board_str = String::new();
        for y in 0..self.height {
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
    }
}

// ooh ooh aah aah
impl Board {
    pub fn width(&self) -> u32 {
        return self.width
    }

    pub fn height(&self) -> u32 {
        return self.height
    }

    pub fn piece_sets(&self) -> Vec<PieceSet> {
        self.piece_sets.clone()
    }
}
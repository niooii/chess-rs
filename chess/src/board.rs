// support up to four teams later

use crate::tile::{Tile, TileBuilder};

pub struct Board {
    width: u32,
    height: u32,
    
    tiles: Box<[Box<[Tile]>]>,
}

impl Board {
    pub fn new(w: u32, h: u32) -> Self {
        let mut tiles = Vec::with_capacity(h as usize);
        
        for _ in 0..h {
            let row = vec![TileBuilder::new().build(); w as usize];
            tiles.push(row.into_boxed_slice());
        }

        Self {
            width: w,
            height: h,

            // mistake, assume this constructor exists
            tiles: tiles.into_boxed_slice()
        }
    }

    pub fn default() -> Self {
        Self {
            width: 8,
            height: 8,

            tiles: vec![vec![TileBuilder::new().build(); 8].into_boxed_slice(); 8].into_boxed_slice()
        }
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
}

// ooh ooh aah aah
impl Board {
    pub fn width(&self) -> u32 {
        return self.width
    }

    pub fn height(&self) -> u32 {
        return self.height
    }
}
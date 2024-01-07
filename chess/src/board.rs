pub struct Board {
    width: u32,
    height: u32,
    
}

impl Board {
    pub fn new(w: u32, h: u32) -> Board {
        Board {
            width: w,
            height: h
        }
    }
}

// ooh ooh aah aah
impl Board {
    pub fn width(&self) {
        return self.width
    }

    pub fn height(&self) {
        return self.height
    }
}
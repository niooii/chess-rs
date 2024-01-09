#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn add(&self, other: &Vec2) -> Self {
        Self {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }

    pub fn mul(&self, n: i32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n
        }
    }
}
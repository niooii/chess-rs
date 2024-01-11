use crate::{team::StartInfo, r#move::Coord, vec2::Vec2};

// will be handled differently depending on team
#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Direction {
    pub fn rel_to_absolute(&self, start_info: StartInfo) -> Self {
        match start_info {
            StartInfo::Bottom { .. } => self.clone(),
            StartInfo::Top { .. } => {
                match self {
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::LeftUp => Direction::RightDown,
                    Direction::LeftDown => Direction::RightUp,
                    Direction::RightUp => Direction::LeftDown,
                    Direction::RightDown => Direction::LeftUp,
                }
            },
            StartInfo::Left { .. } => {
                match self {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::LeftUp => Direction::RightUp,
                    Direction::LeftDown => Direction::LeftUp,
                    Direction::RightUp => Direction::RightDown,
                    Direction::RightDown => Direction::LeftDown,
                }
            },
            StartInfo::Right { .. } => {
                match self {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::LeftUp => Direction::LeftDown,
                    Direction::LeftDown => Direction::RightDown,
                    Direction::RightUp => Direction::LeftUp,
                    Direction::RightDown => Direction::RightUp,
                }
            }
        }
    }

    pub fn as_vec(&self) -> Vec2 {
        match &self {
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
            Direction::Up => Vec2::new(0, 1),
            Direction::Down => Vec2::new(0, -1),
            Direction::LeftUp => Vec2::new(-1, 1),
            Direction::LeftDown => Vec2::new(-1,-1),
            Direction::RightUp => Vec2::new(1, 1),
            Direction::RightDown => Vec2::new(1, -1),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Distance {
    Finite { distance: u32 },
    Infinite,
}

impl Distance {
    pub fn finite(distance: u32) -> Distance {
        Distance::Finite { distance: distance }
    }

    pub fn infinite() -> Distance {
        Distance::Infinite
    }
}

#[derive(Clone, Copy)]
pub struct MoveVec {
    distance: Distance,
    direction: Direction,
}

impl MoveVec {
    pub fn new(distance: Distance, direction: Direction) -> Self {
        Self {
            distance,
            direction,
        }
    }

    pub fn distance(&self) -> Distance {
        self.distance
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn rel_to_absolute(&self, start_info: StartInfo) -> Self {
        Self {
            distance: self.distance,
            direction: self.direction.rel_to_absolute(start_info)
        }
    }
}

#[derive(Clone)]
pub enum MoveRules {
    // a singular jump rule.
    Jump {
        translation: Coord
    },

    // In a direction, piece can jump over every piece/choose one to eat.
    LineJump {
        move_info: Vec<MoveVec>,
    },

    // can pierce through multiple
    // (why would you want this?)
    Pierce {
        move_info: Vec<MoveVec>,
        // maximum amount of points it can pierce through
        max_points: u16,
    },

    // stops after hitting one person
    Blunt {
        move_info: Vec<MoveVec>,
    },

    // FULL radius around the piece.
    Radius {
        tiles: u32,
        // can jump? otherwise, blunt.
    },

    // Try to reduce unnecessary allocations by making this a type.
    KnightJump {
        // distance in left right up down directions to travel.
        radius: u32,
        // distance offset from the ending position, after traveling the radius. (in both directions perpendicular)
        offset: u32,
    },
}

impl MoveRules {
    pub fn jump(translation: Coord) -> MoveRules {
        MoveRules::Jump {
            translation,
        }
    }

    pub fn line_jump(moves: Vec<MoveVec>) -> MoveRules {
        MoveRules::LineJump {
            move_info: moves,
        }
    }

    pub fn pierce(moves: Vec<MoveVec>, max_points: u16) -> MoveRules {
        MoveRules::Pierce {
            max_points,
            move_info: moves,
        }
    }

    pub fn blunt(moves: Vec<MoveVec>) -> MoveRules {
        MoveRules::Blunt { move_info: moves }
    }

    pub fn radius(tiles: u32, can_jump: bool) -> MoveRules {
        MoveRules::Radius {
            tiles,
        }
    }

    pub fn knight_jump(radius: u32, offset: u32) -> MoveRules {
        MoveRules::KnightJump { radius, offset }
    }
}

#[derive(Clone)]
pub struct NthMoveRules {
    move_rules: Vec<MoveRules>,
    // first move is 0 btw
    nth_move: u32,

    // repeats every nth move. why tho?
    // repeating: bool,
}

impl NthMoveRules {
    pub fn new(move_rules: Vec<MoveRules>, nth_move: u32, repeating: bool) -> Self {
        Self {
            move_rules,
            nth_move,
            // repeating,
        }
    }

    pub fn move_rules(&self) -> &Vec<MoveRules> {
        &self.move_rules
    }

    // pub fn repeating(&self) -> bool {
    //     self.repeating
    // }

    pub fn nth_move(&self) -> u32 {
        self.nth_move
    }
}

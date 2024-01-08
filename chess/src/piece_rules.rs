
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
    RightDown
}

#[derive(Clone, Copy)]
pub enum Distance {
    Finite {
        distance: u32
    },
    Infinite
}

impl Distance {
    pub fn finite(distance: u32) -> Distance {
        Distance::Finite { distance: 
            distance
        }
    }

    pub fn infinite() -> Distance {
        Distance::Infinite
    }
}

#[derive(Clone, Copy)]
pub struct MoveVec {
    distance: Distance,
    direction: Direction
}

impl MoveVec {
    pub fn new(distance: Distance, direction: Direction) -> Self {
        Self {
            distance,
            direction
        }
    }
}

#[derive(Clone)]
pub enum MoveRules {
    // jumps tiles (knight type movement)
    Jump {
        move_info: Vec<MoveVec>,
        // if this is true, it is applied once after processing all the MoveVecs.
        // otherwise, applies a jump for every MoveVec.
        combine_directions: bool
    },

    // can pierce through multiple
    // (why would you want this?)
    Pierce {
        move_info: Vec<MoveVec>,
        // maximum amount of points it can pierce through
        max_points: u16
    },
    
    // stops after hitting one person
    Blunt {
        move_info: Vec<MoveVec>
    },

    // FULL radius around the piece.
    Radius {
        tiles: u32,
        // can jump? otherwise, blunt.
        // pierce doesnt make much sense with this. I am not solving another rasterization problem
        jump: bool
    },

    // Try to reduce unnecessary allocations by making this a type. 
    KnightJump {
        // distance in left right up down directions to travel.
        radius: u32,
        // distance offset from the ending position, after traveling the radius. (in both directions perpendicular)
        offset: u32
    }
}

impl MoveRules {
    pub fn jump(moves: Vec<MoveVec>, combine_directions: bool) -> MoveRules {
        MoveRules::Jump { combine_directions, move_info: moves }
    }

    pub fn pierce(moves: Vec<MoveVec>, max_points: u16) -> MoveRules {
        MoveRules::Pierce { max_points, move_info: moves }
    }
    
    pub fn blunt(moves: Vec<MoveVec>) -> MoveRules {
        MoveRules::Blunt { move_info: moves }
    }

    pub fn radius(tiles: u32, can_jump: bool) -> MoveRules {
        MoveRules::Radius { tiles, jump: can_jump }
    }

    pub fn knight_jump(radius: u32, offset: u32) -> MoveRules {
        MoveRules::KnightJump {radius, offset}
    }
}

#[derive(Clone)]
pub struct NthMoveRules {
    move_rules: Vec<MoveRules>,
    nth_move: u32,
    
    // repeats every nth move. why tho?
    repeating: bool
}

impl NthMoveRules {
    pub fn new(move_rules: Vec<MoveRules>, nth_move: u32, repeating: bool) -> Self {
        Self {
            move_rules,
            nth_move,
            repeating
        }
    }
}
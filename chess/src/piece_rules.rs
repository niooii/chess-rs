
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDownsy
}

pub struct MoveVec {
    distance: u32,
    direction: Direction
}

pub enum MoveRules {
    // jumps tiles (knight type movement)
    Jump {
        // if this is true, it is applied once after processing all the MoveVecs.
        // otherwise, applies a jump for every MoveVec.
        applied_once: bool,
        move_info: Vec<MoveVec>
    },

    // can pierce through multiple
    // (why would you want this?)
    Pierce {
        move_info: Vec<MoveVec>
    },
    
    // stops after hitting one person
    Blunt {
        move_info: Vec<MoveVec>
    }

}

impl MoveRules {
    pub fn Jump(applied_once: bool, moves: Vec<MoveVec>) -> Jump {

    }
}
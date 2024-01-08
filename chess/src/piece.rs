use std::sync::{RwLock, Arc};

use crate::{piece_rules::{MoveRules, NthMoveRules}};

/// custom type to reduce boilerplate
/// piece uses arc internally
pub type Piece = Arc<RwLock<PieceRef>>;

/// creates a new copy of the underlying data.
pub fn clone_piece(original: Piece) -> Piece {
    Arc::new(
        RwLock::new(
            original.read().unwrap().clone()
        )
    )
}

#[derive(Clone)]
pub struct PieceRef {
    name: String,
    points: u16,
    move_rules: Vec<MoveRules>,
    kill_rules: Vec<MoveRules>,
    // applies on the nth move of the piece (start from 1, not 0)
    nth_move_rules: Vec<NthMoveRules>,
    
    alive: bool,
    jump_immune: bool,
    pierce_immune: bool,

    // allows kill move rules to be used for moving
    use_kill_for_moves: bool,
    // allows default move rules (AND nth move rules) to be used for killing
    use_moves_for_kills: bool
}

impl PieceRef {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn points(&self) -> u16 {
        self.points
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_jump_immune(&self) -> bool {
        self.jump_immune
    }

    pub fn is_pierce_immune(&self) -> bool {
        self.pierce_immune
    }

    pub fn can_use_kill_for_moves(&self) -> bool {
        self.use_kill_for_moves
    }
    
    pub fn can_use_moves_for_kills(&self) -> bool {
        self.use_moves_for_kills
    }
}

#[derive(Default)]
pub struct PieceBuilder {
    name: String,
    points: u16,
    move_rules: Vec<MoveRules>,
    kill_rules: Vec<MoveRules>,
    // applies on the nth move of the piece (start from 1, not 0)
    nth_move_rules: Vec<NthMoveRules>,
    
    jump_immune: bool,
    pierce_immune: bool,

    // allows kill move rules to be used for moving
    use_kill_for_moves: bool,
    // allows default move rules (AND nth move rules) to be used for killing
    use_moves_for_kills: bool
}

impl PieceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn points(mut self, points: u16) -> Self {
        self.points = points;
        self
    }

    pub fn move_rules(mut self, move_rules: Vec<MoveRules>) -> Self {
        self.move_rules = move_rules;
        self
    }

    pub fn kill_rules(mut self, kill_rules: Vec<MoveRules>) -> Self {
        self.kill_rules = kill_rules;
        self
    }

    pub fn nth_move_rules(mut self, nth_move_rules: Vec<NthMoveRules>) -> Self {
        self.nth_move_rules = nth_move_rules;
        self
    }

    pub fn jump_immune(mut self, jump_immune: bool) -> Self {
        self.jump_immune = jump_immune;
        self
    }

    pub fn pierce_immune(mut self, pierce_immune: bool) -> Self {
        self.pierce_immune = pierce_immune;
        self
    }

    pub fn use_kill_for_moves(mut self, use_kill_for_moves: bool) -> Self {
        self.use_kill_for_moves = use_kill_for_moves;
        self
    }

    pub fn use_moves_for_kills(mut self, use_moves_for_kills: bool) -> Self {
        self.use_moves_for_kills = use_moves_for_kills;
        self
    }

    pub fn build(mut self) -> Piece {
        Arc::new(
            RwLock::new(
                PieceRef {
                    name: self.name,
                    points: self.points,
                    move_rules: self.move_rules,
                    kill_rules: self.kill_rules,
                    nth_move_rules: self.nth_move_rules,
                    alive: true,
                    jump_immune: self.jump_immune,
                    pierce_immune: self.pierce_immune,
                    use_kill_for_moves: self.use_kill_for_moves,
                    use_moves_for_kills: self.use_moves_for_kills
                }
            )
        )
    }
}

pub mod defaults {
    use crate::{piece::PieceRef, piece_rules::{MoveRules, MoveVec, Direction, Distance, NthMoveRules}};

    use super::{PieceBuilder, Piece};

    pub fn pawn() -> Piece {
        let move_rules = vec![
            MoveRules::blunt(
                vec![MoveVec::new(Distance::finite(1), Direction::Up)]
            )
        ];

        let kill_rules = vec![
            MoveRules::blunt(
                vec![MoveVec::new(Distance::finite(1), Direction::LeftUp)]
            ),
            MoveRules::blunt(
                vec![MoveVec::new(Distance::finite(1), Direction::RightUp)]
            )
        ];

        let nth_move_rules = vec![
            NthMoveRules::new(
                vec![
                    MoveRules::blunt(
                        vec![MoveVec::new(Distance::finite(2), Direction::Up)]
                    )
                ],
                1, 
                false
            )
        ];

        PieceBuilder::new()
            .name("Pawn".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .build()
    }
    
    pub fn rook() -> Piece {
        let move_rules = vec![
            MoveRules::blunt(
                vec![
                    MoveVec::new(Distance::Infinite, Direction::Up),
                    MoveVec::new(Distance::Infinite, Direction::Down),
                    MoveVec::new(Distance::Infinite, Direction::Left),
                    MoveVec::new(Distance::Infinite, Direction::Right)
                ]
            )
        ];

        let kill_rules = Vec::new();

        let nth_move_rules = Vec::new();

        PieceBuilder::new()
            .name("Rook".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .use_moves_for_kills(true)
            .build()
    }

    pub fn bishop() -> Piece {
        let move_rules = vec![
            MoveRules::blunt(
                vec![
                    MoveVec::new(Distance::Infinite, Direction::LeftUp),
                    MoveVec::new(Distance::Infinite, Direction::LeftDown),
                    MoveVec::new(Distance::Infinite, Direction::RightUp),
                    MoveVec::new(Distance::Infinite, Direction::RightDown)
                ]
            )
        ];

        let kill_rules = Vec::new();

        let nth_move_rules = Vec::new();

        PieceBuilder::new()
            .name("Bishop".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .use_moves_for_kills(true)  
            .build()
    }

    pub fn knight() -> Piece {
        let move_rules = vec![
            MoveRules::knight_jump(2, 1)
        ];
        
        let kill_rules = Vec::new();

        let nth_move_rules = Vec::new();

        PieceBuilder::new()
            .name("Knight".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .use_moves_for_kills(true)
            .build()
    }

    pub fn queen() -> Piece {
        let move_rules = vec![
            MoveRules::blunt(
                vec![
                    MoveVec::new(Distance::Infinite, Direction::Up),
                    MoveVec::new(Distance::Infinite, Direction::Down),
                    MoveVec::new(Distance::Infinite, Direction::Left),
                    MoveVec::new(Distance::Infinite, Direction::Right),
                    MoveVec::new(Distance::Infinite, Direction::LeftUp),
                    MoveVec::new(Distance::Infinite, Direction::LeftDown),
                    MoveVec::new(Distance::Infinite, Direction::RightUp),
                    MoveVec::new(Distance::Infinite, Direction::RightDown)
                ]
            )
        ];

        let kill_rules = Vec::new();

        let nth_move_rules = Vec::new();

        PieceBuilder::new()
            .name("Queen".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .use_moves_for_kills(true)
            .build()
    }

    pub fn king() -> Piece {
        let move_rules = vec![
            MoveRules::radius(1, false)
        ];

        let kill_rules = Vec::new();

        let nth_move_rules = Vec::new();

        PieceBuilder::new()
            .name("King".to_string())
            .points(1)
            .move_rules(move_rules)
            .kill_rules(kill_rules)
            .nth_move_rules(nth_move_rules)
            .use_moves_for_kills(true)
            .build()
    }
}
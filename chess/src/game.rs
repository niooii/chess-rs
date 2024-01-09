use std::sync::Arc;

use crate::board::Board;
use crate::error::{ChessError, Result};
use crate::piece::{self, Piece, PieceBuilder};
use crate::piece_rules::MoveRules;
use crate::piece_set::PieceSet;
use crate::r#move::{Coord, Move};
use crate::team::{StartInfo, Team};

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        todo!();
        // Self {
        //     board: todo!()
        // }
    }

    pub fn original() -> Result<Self> {
        let mut board = Board::new(8, 8);
        let white_team = Team::new("White".to_string(), StartInfo::Bottom { offset: 0 });
        let black_team = Team::new("Black".to_string(), StartInfo::Top { offset: 0 });

        let white = Arc::new(white_team);
        let black = Arc::new(black_team);

        let mut white_set = PieceSet::new(white.clone(), Vec::new());

        let white_pawn = piece::defaults::pawn();

        for i in 0..8 {
            white_set.add_piece(PieceBuilder::clone_piece(&white_pawn), Coord::new(i, 1))?;
        }

        white_set.add_piece(piece::defaults::king(), Coord::new(4, 0))?;
        white_set.add_piece(piece::defaults::queen(), Coord::new(3, 0))?;
        white_set.add_piece(piece::defaults::rook(), Coord::new(0, 0))?;
        white_set.add_piece(piece::defaults::rook(), Coord::new(7, 0))?;
        white_set.add_piece(piece::defaults::bishop(), Coord::new(2, 0))?;
        white_set.add_piece(piece::defaults::bishop(), Coord::new(5, 0))?;
        white_set.add_piece(piece::defaults::knight(), Coord::new(1, 0))?;
        white_set.add_piece(piece::defaults::knight(), Coord::new(6, 0))?;

        let black_set = white_set.clone_for_team(black);

        board.add_piece_set(white_set)?;
        board.add_piece_set(black_set)?;

        board.init()?;

        Ok(Self { board })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}

// game logic implementations
// TODO: make piece an actual struct instead of a type alias, and then give the struct the readonly properties and then have an
// internal field to hold the pieceref. (EX: team should not be mutable.)
impl Game {
    pub fn validate_game(&self) -> Result<()> {
        for set in self.board().piece_sets() {
            for piece in set.pieces() {
                let read_lock = piece.read().unwrap();
                if read_lock.team().is_none() {
                    return Err(ChessError::GameValidationError {
                        why: format!("Some piece wasn't assigned a team: {}", read_lock.name()),
                    });
                }
                if read_lock.rel_pos().is_none() {
                    return Err(ChessError::GameValidationError { 
                        why: format!("Some piece was not assigned a relative position: {}", read_lock.name())
                    })
                }
            }
        }

        Ok(())
    }

    pub fn calculate_moves_for(&self, team_name: String) -> Result<Vec<Move>> {
        let mut moves = Vec::<Move>::new();

        let board = &self.board;
        let piece_sets = &board.piece_sets();

        let set = piece_sets.iter().find(|set| set.team().name() == team_name);

        if set.is_none() {
            return Err(ChessError::MoveCalculationError { why: format!("No team with name {}", team_name) });
        }

        let set = set.unwrap();
        let team = set.team();

        for piece in set.pieces() {
            let r_lock = piece.read().unwrap();

            // handle regular movemenet rules
            for rule in r_lock.move_rules() {
                match rule {
                    MoveRules::Jump { translation } => {
                        todo!();
                    },
                    MoveRules::LineJump { move_info } => {
                        for move_vec in move_info {
                            let move_vec = move_vec.rel_to_absolute(team.start_info());
                            
                            
                        }
                    }
                    MoveRules::Pierce { move_info, max_points } => {

                    },
                    MoveRules::Blunt { move_info } => todo!(),
                    MoveRules::Radius { tiles, jump } => todo!(),
                    MoveRules::KnightJump { radius, offset } => todo!(),
                }
            }
        }

        Ok(moves)
    }
}

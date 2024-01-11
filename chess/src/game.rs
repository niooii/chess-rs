use std::sync::Arc;

use crate::board::Board;
use crate::error::{ChessError, Result};
use crate::piece::{self, Piece, PieceBuilder};
use crate::piece_rules::{MoveRules, Distance, Direction};
use crate::piece_set::PieceSet;
use crate::r#move::{Move, Coord};
use crate::team::{StartInfo, Team};
use crate::vec2::Vec2;

pub struct Game {
    board: Board,
    current_move: u32
}

impl Game {
    pub fn new() -> Self {
        todo!();
        // Self {
        //     board: todo!()
        // }
    }

    pub fn original() -> Result<Self> {
        let mut board = Board::new(10, 10);
        let white_team = Team::new("White".to_string(), StartInfo::Bottom { offset: 1 });
        let black_team = Team::new("Black".to_string(), StartInfo::Top { offset: 1 });

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

        Ok(
            Self {
                board, 
                current_move: 0 
            }
        )
    }

    pub fn two_piece_test(piece: Piece) -> Result<Self> {
        let mut board = Board::new(10, 10);

        let white = Arc::new(Team::new("White".to_string(), StartInfo::Bottom { offset: 0 }));
        let black = Arc::new(Team::new("Black".to_string(), StartInfo::Top { offset: 0 }));

        let mut white_set = PieceSet::new(white.clone(), Vec::new());
        white_set.add_piece(piece, Coord::new(2, 3))?;

        let black_set = white_set.clone_for_team(black);

        board.add_piece_set(white_set)?;
        board.add_piece_set(black_set)?;

        Ok(
            Self {
                board,
                current_move: 0
            }
        )
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_move(&self) -> u32 {
        self.current_move
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

    // diabolical logic implementation
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
                
            }
        }

        Ok(moves)
    }

    fn calc_moves(move_rule: &MoveRules, moves: &mut Vec<Move>) {
        match move_rule {
            MoveRules::Jump { translation } => {
                todo!();
            },
            MoveRules::LineJump { move_info } => {
                todo!();
            }
            MoveRules::Pierce { move_info, max_points } => {
                todo!();
            },
            MoveRules::Blunt { move_info } => {
                'process_move_vecs: for move_vec in move_info {
                    let start_info = team.start_info();
                    let abs_move_vec = move_vec.rel_to_absolute(start_info);
                    let rel_pos = r_lock.rel_pos_unchecked();
                    
                    let abs_pos = board.rel_coord_to_absolute(rel_pos, start_info);

                    let offset_vec = abs_move_vec.direction().as_vec();
                    
                    match abs_move_vec.distance() {
                        Distance::Finite { distance } => {
                            for n in 1..=distance {
                                let target_coord = abs_pos.translate(&(offset_vec.mul(n as i32)));
                                // if this triggers, we are out of bounds on the negative direction.
                                if target_coord.is_err() {
                                    continue 'process_move_vecs;
                                }

                                let target_coord = target_coord.unwrap();

                                // if tile is none, we are out of bounds on the positive direction.
                                if let Some(t) = board.tile_at(target_coord.x(), target_coord.y()) {
                                    // IMPLEMENT REL_TRANSLATION LATER IMPORTANT TODO!
                                    moves.push(
                                        Move::new(piece.clone(), abs_pos, target_coord, Coord::new(0,0), 
                                        board.tile_at(abs_pos.x(), abs_pos.y()).unwrap(), 
                                        t.clone())
                                    );

                                    // if there was a piece here stop and go to next direction or something
                                    if t.read().unwrap().occupied() {
                                        continue 'process_move_vecs;
                                    }
                                } else {
                                    continue 'process_move_vecs;
                                }
                            }
                        },
                        Distance::Infinite => {
                            let mut idx = 1_u32;
                            let mut target_coord = abs_pos.translate(&offset_vec.mul(idx as i32));

                            while target_coord.is_ok() {

                                let target_coord_inner = target_coord.unwrap();

                                // if tile is none, we are out of bounds on the positive direction.
                                if let Some(t) = board.tile_at(target_coord_inner.x(), target_coord_inner.y()) {
                                    // IMPLEMENT REL_TRANSLATION LATER IMPORTANT TODO!
                                    moves.push(
                                        Move::new(piece.clone(), abs_pos, target_coord_inner, Coord::new(0,0), 
                                        board.tile_at(abs_pos.x(), abs_pos.y()).unwrap(), 
                                        t.clone())
                                    );

                                    // if there was a piece here stop and go to next direction or something
                                    if t.read().unwrap().occupied() {
                                        continue 'process_move_vecs;
                                    }
                                } else {
                                    continue 'process_move_vecs;
                                }

                                idx += 1;
                                target_coord = abs_pos.translate(&offset_vec.mul(idx as i32));
                            }
                        },
                    }
                }
            },
            MoveRules::Radius { tiles, jump } => {

            },
            MoveRules::KnightJump { radius, offset } => {
                
            },
        }
    }
}


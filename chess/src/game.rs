use std::sync::Arc;

use crate::board::Board;
use crate::error::{ChessError, Result};
use crate::piece::{self, Piece, PieceBuilder};
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
        let mut board = Board::new(10, 10);
        let white_team = Team::new("White".to_string(), StartInfo::Bottom { offset: 2 });
        let black_team = Team::new("Black".to_string(), StartInfo::Top { offset: 1 });

        let white = Arc::new(white_team);
        let black = Arc::new(black_team);

        let mut white_set = PieceSet::new(white.clone(), Vec::new());
        let mut black_set = PieceSet::new(black.clone(), Vec::new());

        let white_pawn = piece::defaults::pawn();
        let black_pawn = piece::defaults::pawn();

        // Add white pawns
        for i in 0..8 {
            white_set.add_piece(PieceBuilder::clone_piece(&white_pawn), Coord::new(i, 1))?;
        }

        // Add black pawns
        for i in 0..8 {
            black_set.add_piece(PieceBuilder::clone_piece(&black_pawn), Coord::new(i, 1))?;
        }

        white_set.add_piece(piece::defaults::king(), Coord::new(4, 0))?;
        white_set.add_piece(piece::defaults::queen(), Coord::new(3, 0))?;
        white_set.add_piece(piece::defaults::rook(), Coord::new(0, 0))?;
        white_set.add_piece(piece::defaults::rook(), Coord::new(7, 0))?;
        white_set.add_piece(piece::defaults::bishop(), Coord::new(2, 0))?;
        white_set.add_piece(piece::defaults::bishop(), Coord::new(5, 0))?;
        white_set.add_piece(piece::defaults::knight(), Coord::new(1, 0))?;
        white_set.add_piece(piece::defaults::knight(), Coord::new(6, 0))?;

        // Add black pieces

        black_set.add_piece(piece::defaults::king(), Coord::new(4, 0))?;
        black_set.add_piece(piece::defaults::queen(), Coord::new(3, 0))?;
        black_set.add_piece(piece::defaults::rook(), Coord::new(0, 0))?;
        black_set.add_piece(piece::defaults::rook(), Coord::new(7, 0))?;
        black_set.add_piece(piece::defaults::bishop(), Coord::new(2, 0))?;
        black_set.add_piece(piece::defaults::bishop(), Coord::new(5, 0))?;
        black_set.add_piece(piece::defaults::knight(), Coord::new(1, 0))?;
        black_set.add_piece(piece::defaults::knight(), Coord::new(6, 0))?;

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
    fn validate_game(&self) -> Result<()> {
        for set in self.board().piece_sets() {
            for piece in set.pieces() {
                let read_lock = piece.read().unwrap();
                if read_lock.team().is_none() {
                    return Err(ChessError::GameValidationError {
                        why: format!("Some piece wasn't assigned a team: {}", read_lock.name()),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn calculate_moves_for(team_name: String) -> Result<Vec<Move>> {
        let mut moves = Vec::<Move>::new();

        Ok(moves)
    }
}

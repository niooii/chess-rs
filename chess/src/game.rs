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
        let mut board = Board::new(12, 12);
        let white_team = Team::new("White".to_string(), StartInfo::Bottom { offset: 2 });
        let black_team = Team::new("Black".to_string(), StartInfo::Top { offset: 2 });
        let blue_team = Team::new("Blue".to_string(), StartInfo::Left { offset: 2 });
        let green_team = Team::new("Green".to_string(), StartInfo::Right { offset: 2 });

        let white = Arc::new(white_team);
        let black = Arc::new(black_team);
        let blue = Arc::new(blue_team);
        let green = Arc::new(green_team);

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
        let green_set = white_set.clone_for_team(green);
        let blue_set = white_set.clone_for_team(blue);

        board.add_piece_set(white_set)?;
        board.add_piece_set(black_set)?;
        board.add_piece_set(green_set)?;
        board.add_piece_set(blue_set)?;

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

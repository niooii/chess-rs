use std::sync::Arc;

use crate::board::Board;
use crate::error::{Result, ChessError};
use crate::piece::{Piece, PieceBuilder, self};
use crate::piece_set::PieceSet;
use crate::r#move::Move;
use crate::team::{Team, StartInfo};


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

        let white_set = PieceSet::new(white.clone(), Vec::new());
        let black_set = PieceSet::new(black.clone(), Vec::new());

        let white_pawn = piece::defaults::pawn(white.clone());
        let black_pawn = piece::defaults::pawn(black.clone());

        // Add white pawns
        for i in 0..8 {
            board.add_piece_at(PieceBuilder::clone_piece(&white_pawn), board.tile_at(i, 6).unwrap())?;
        }

        // Add black pawns
        for i in 0..8 {
            board.add_piece_at(PieceBuilder::clone_piece(&black_pawn), board.tile_at(i, 1).unwrap())?;
        }
    
        // Add white pieces
        let white_king = piece::defaults::king(white.clone());
        let white_queen = piece::defaults::queen(white.clone());
        let white_rook = piece::defaults::rook(white.clone());
        let white_bishop = piece::defaults::bishop(white.clone());
        let white_knight = piece::defaults::knight(white.clone());

        board.add_piece_at(PieceBuilder::clone_piece(&white_king), board.tile_at(4, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_queen), board.tile_at(3, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_rook), board.tile_at(0, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_rook), board.tile_at(7, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_bishop), board.tile_at(2, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_bishop), board.tile_at(5, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_knight), board.tile_at(1, 0).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&white_knight), board.tile_at(6, 0).unwrap())?;

        // Add black pieces
        let black_king = piece::defaults::king(black.clone());
        let black_queen = piece::defaults::queen(black.clone());
        let black_rook = piece::defaults::rook(black.clone());
        let black_bishop = piece::defaults::bishop(black.clone());
        let black_knight = piece::defaults::knight(black.clone());

        board.add_piece_at(PieceBuilder::clone_piece(&black_king), board.tile_at(4, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_queen), board.tile_at(3, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_rook), board.tile_at(0, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_rook), board.tile_at(7, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_bishop), board.tile_at(2, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_bishop), board.tile_at(5, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_knight), board.tile_at(1, 7).unwrap())?;
        board.add_piece_at(PieceBuilder::clone_piece(&black_knight), board.tile_at(6, 7).unwrap())?;

        Ok(
            Self {
                board,
            }
        )
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
                    return Err(ChessError::GameValidationError { why: format!("Some piece wasn't assigned a team: {}", read_lock.name())});
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
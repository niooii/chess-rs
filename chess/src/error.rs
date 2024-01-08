use thiserror::Error;

pub type Result<T> = std::result::Result<T, ChessError>;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("Error with tile.")]
    TileActionError {
        why: String
    },
    
    #[error("Error creating piece.")]
    PieceCreationError {
        why: String
    }
    
    #[error("Error validating game.")]
    GameValidationError {
        why: String
    }
}
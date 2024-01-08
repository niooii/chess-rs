use thiserror::Error;

pub type Result<T> = std::result::Result<T, ChessError>;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("Error with tile.")]
    TileActionError {
        why: String
    }
}
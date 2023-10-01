use thiserror::Error;

#[derive(Error, Debug)]
pub enum SurfelReconstError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String)
}

pub type SurfelReconsResult<T> = std::result::Result<T, SurfelReconstError>;
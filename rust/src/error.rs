#![allow(dead_code)]

use crate::{input::error::InputError, solution::error::SolutionError};

pub type Result<T> = core::result::Result<T, AoCError>;

#[derive(thiserror::Error, Debug)]
#[error("AoC Error")]
pub enum AoCError {
    #[error(transparent)]
    Solution(#[from] SolutionError),
    #[error(transparent)]
    Input(#[from] InputError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

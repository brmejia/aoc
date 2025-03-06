pub type Result<T> = core::result::Result<T, SolutionError>;

#[derive(thiserror::Error, Debug)]
#[error("Solution Error")]
pub enum SolutionError {
    #[error("Year {year} is not implemented yet")]
    NotImplementedYear { year: u16 },
    #[error("Day {day} of {year} is not implemented yet")]
    NotImplementedDay { year: u16, day: u8 },
    #[error("Day {day} is not a valid Advent of Code")]
    InvalidDay { day: u8 },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

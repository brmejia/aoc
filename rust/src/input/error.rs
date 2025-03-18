pub type Result<T> = core::result::Result<T, InputError>;

#[derive(thiserror::Error, Debug)]
#[error("Problem Input Error")]
pub enum InputError {
    IOError(#[from] std::io::Error),
    #[error("Error parsing input: {0}")]
    Parsing(String),
}

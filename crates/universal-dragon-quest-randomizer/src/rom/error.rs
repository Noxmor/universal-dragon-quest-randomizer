#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read ROM: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown ROM")]
    UnknownRom,

    #[error("unknown format")]
    UnknownFormat,

    #[error("ROM is empty")]
    Empty,
}

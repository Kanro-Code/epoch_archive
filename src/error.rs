use thiserror::Error;

#[derive(Error, Debug)]
pub enum Codec {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("RMP Encode Error")]
    SerdeError(#[from] rmp_serde::encode::Error),
    #[error("RMP Decode Error")]
    SerdeDecodeError(#[from] rmp_serde::decode::Error),
}
#[derive(Error, Debug)]
pub enum Epoch {
    #[error("Invalid SubSecond: {0}")]
    InvalidSubSecond(String),
    #[error("ParseIntError")]
    InvalidEpoch(#[from] std::num::ParseIntError),
}

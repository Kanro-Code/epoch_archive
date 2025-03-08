#![crate_name = "epoch_archive"]
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

mod codec;
mod epoch;
mod error;

pub use codec::Codec;
pub use epoch::Epoch;
pub use epoch::SubSecond;

pub use error::Codec as CodecError;

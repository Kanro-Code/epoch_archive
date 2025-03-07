#![crate_name = "epoch_archive"]
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

mod epoch;
pub use epoch::Epoch;
pub use epoch::SubSecond;

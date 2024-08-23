// Modules

mod card;
mod set;
mod rarities;
mod subtypes;
mod supertypes;
mod types;

mod client;
mod error;

mod utils;

// Flatten
pub use client::*;
pub use card::*;
pub use set::*;
pub use error::{Error, Result};

pub(crate) use utils::Requestable;

// Public Modules
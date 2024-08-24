// Modules

pub mod card;
pub mod set;

mod client;
mod error;

mod utils;

// Flatten
pub use client::*;
pub use card::Card;
pub use set::Set;
pub use error::{Error, Result};

pub(crate) use utils::Requestable;

// Public Modules
pub mod rarities;
pub mod subtypes;
pub mod supertypes;
pub mod types;
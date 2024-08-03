// Modules

mod models;

mod client;
mod error;

// Flatten
pub use client::*;
pub use error::{Error, Result};

// Public Modules
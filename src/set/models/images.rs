use serde::{Deserialize, Serialize};

/// Holds the logo and symbol image URLs for a set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetImages {
	pub symbol: String,
	pub logo: String,
}
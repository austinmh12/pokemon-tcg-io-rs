use serde::{Deserialize, Serialize};

/// Holds the logo and symbol image URLs for a set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetImages {
	/// The url to the symbol image.
	pub symbol: String,
	/// The url to the logo image.
	pub logo: String,
}
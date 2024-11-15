use serde::{Deserialize, Serialize};

/// Hold the URLs to a card's normal and hi-res images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImages {
	/// A smaller, lower-res image for a card. This is a URL.
	pub small: String,
	/// A larger, higher-res image for a card. This is a URL.
	pub large: String,
}
use serde::{Deserialize, Serialize};

/// Hold the URLs to a card's normal and hi-res images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImages {
	pub small: String,
	pub large: String,
}
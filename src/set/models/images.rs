use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetImages {
	pub symbol: String,
	pub logo: String,
}
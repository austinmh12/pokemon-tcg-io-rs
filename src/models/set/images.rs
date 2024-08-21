use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
	pub symbol: String,
	pub logo: String,
}
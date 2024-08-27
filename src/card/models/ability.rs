use serde::{Deserialize, Serialize};

/// Represents an ability on a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
	pub name: String,
	pub text: String,
	#[serde(rename = "type")]
	pub ability_type: String,
}
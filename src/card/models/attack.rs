use serde::{Deserialize, Serialize};

/// Represents an attack on a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
	pub cost: Vec<String>,
	pub name: String,
	pub text: String,
	pub damage: Option<String>,
	pub converted_energy_cost: i32,
}
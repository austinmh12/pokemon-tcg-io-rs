use serde::{Deserialize, Serialize};

/// Represents an ability on a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
	/// The name of the ability.
	pub name: String,
	/// The text value of the ability.
	pub text: String,
	/// The type of the ability, such as Ability or Pokémon-Power.
	pub r#type: String,
}
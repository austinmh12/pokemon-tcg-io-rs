use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
	pub id: String,
	pub name: String,
	pub supertype: String,
	pub subtypes: Option<Vec<String>>,
	pub level: Option<String>,
	pub hp: Option<String>,
	pub types: Option<Vec<String>>,
	pub evolves_from: Option<String>,
	pub evolves_to: Option<Vec<String>>,
	pub rules: Option<Vec<String>>,
	pub ancient_trait: Option<HashMap<String, String>>,
	pub abilities: Option<Vec<Ability>>,
	pub attacks: Option<Vec<Attack>>,
	pub weaknesses: Option<Vec<HashMap<String, String>>>,
	pub resistances: Option<Vec<HashMap<String, String>>>,
	pub retreat_cost: Option<Vec<String>>,
	pub converted_retreat_cost: Option<i32>,
	// Skip for now
	// pub set: Set,
	pub number: String,
	pub artist: String,
	pub rarity: Option<String>,
	pub flavor_text: Option<String>,
	pub national_pokedex_numbers: Option<Vec<i32>>,
	pub legalities: HashMap<String, String>,
	pub regulation_mark: Option<String>,
	pub images: Images,
	// Skip for now
	// pub tcgplayer: TCGPlayer
	// pub cardmarket: CardMarket
}

impl PartialEq for Card {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Card {}

/// Card Specific Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
	pub name: String,
	pub text: String,
	#[serde(rename = "type")]
	pub ability_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
	pub cost: Vec<String>,
	pub name: String,
	pub text: String,
	pub damage: String,
	pub converted_energy_cost: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
	pub small: String,
	pub large: String,
}
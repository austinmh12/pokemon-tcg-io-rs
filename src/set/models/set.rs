use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::SetImages;

/// Represents a printed set of cards from the pokemontcg.io REST API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
	/// Unique identifier for the object.
	pub id: String,
	/// The name of the set.
	pub name: Option<String>,
	/// The series the set belongs to, like Sword and Shield or Base.
	pub series: Option<String>,
	/// The number printed on the card that represents the total. This total does not include secret rares.
	pub printed_total: Option<i64>,
	/// The total number of cards in the set, including secret rares, alternate art, etc.
	pub total: Option<i64>,
	/// The legalities of the set. If a given format is not legal, it will not appear in the hash.
	pub legalities: Option<HashMap<String, String>>,
	/// The code the Pokémon Trading Card Game Online uses to identify a set.
	pub ptcgo_code: Option<String>,
	/// The date the set was released (in the USA). Format is YYYY/MM/DD.
	pub release_date: Option<String>,
	/// The date and time the set was updated. Format is YYYY/MM/DD HH:MM:SS.
	pub updated_at: Option<String>,
	/// Any images associated with the set, such as symbol and logo.
	pub images: Option<SetImages>,
}

impl PartialEq for Set {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Set {}
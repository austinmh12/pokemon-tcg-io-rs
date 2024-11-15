use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::*;
use crate::Set;

/// Represents a Pokémon card from the pokemontcg.io REST API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
	/// Unique identifier for the card.
	pub id: String,
	/// The name of the card
	pub name: Option<String>,
	/// The supertype of the card, such as Pokémon, Energy, or Trainer.
	pub supertype: Option<String>,
	/// A list of subtypes, such as Basic, EX, Mega, Rapid Strike, etc.
	pub subtypes: Option<Vec<String>>,
	/// The level of the card. This only pertains to cards from older sets and those of supertype Pokémon.
	pub level: Option<String>,
	/// The hit points of the card.
	pub hp: Option<String>,
	/// The energy types for a card, such as Fire, Water, Grass, etc.
	pub types: Option<Vec<String>>,
	/// Which Pokémon this card evolves from.
	pub evolves_from: Option<String>,
	/// Which Pokémon this card evolves to. Can be multiple, for example, Eevee.
	pub evolves_to: Option<Vec<String>>,
	/// Any rules associated with the card. For example, VMAX rules, Mega rules, or various trainer rules.
	pub rules: Option<Vec<String>>,
	/// The ancient trait for a given card. An ancient trait has the following fields:
	/// 
	/// name - String - The name of the ancient trait.
	/// 
	/// test - String - The text value of the ancient trait.
	pub ancient_trait: Option<HashMap<String, String>>,
	/// One or more abilities for a given card.
	pub abilities: Option<Vec<Ability>>,
	/// One or more attacks for a given card.
	pub attacks: Option<Vec<Attack>>,
	/// One or more weaknesses for a given card. A weakness has the following fields:
	/// 
	/// type - String - The type of weakness, such as Fire or Water.
	/// 
	/// value - String - The value of the weakness.
	pub weaknesses: Option<Vec<HashMap<String, String>>>,
	/// One or more resistances for a given card. A resistance has the following fields:
	/// 
	/// type - String - The type of resistance, such as Fire or Water.
	/// 
	/// value - String - The value of the resistance.
	pub resistances: Option<Vec<HashMap<String, String>>>,
	/// A list of costs it takes to retreat and return the card to your bench. Each cost is an energy type, such as Water or Fire.
	pub retreat_cost: Option<Vec<String>>,
	/// The converted retreat cost for a card is the count of energy types found within the retreat_cost field. For
	/// example, ["Fire", "Water"] has a converted retreat cost of 2.
	pub converted_retreat_cost: Option<i32>,
	/// The `Set` the card belongs to.
	pub set: Option<Set>,
	/// The number of the card.
	pub number: Option<String>,
	/// The artist of the card.
	pub artist: Option<String>,
	/// The rarity of the card, such as "Common" or "Rare Rainbow".
	pub rarity: Option<String>,
	/// The flavor text of the card. This is the text that can be found on some Pokémon cards
	/// that is usually italicized near the bottom of the card.
	pub flavor_text: Option<String>,
	/// The national pokedex numbers associated with any Pokémon featured on a given card.
	pub national_pokedex_numbers: Option<Vec<i32>>,
	/// The legalities for a given card. A legality will not be present in the hash if it is not legal. 
	/// If it is legal or banned, it will be present.
	pub legalities: Option<HashMap<String, String>>,
	/// A letter symbol found on each card that identifies whether it is legal to use in tournament play. 
	/// Regulation marks were introduced on cards in the Sword & Shield Series.
	pub regulation_mark: Option<String>,
	/// The images for a card.
	pub images: Option<CardImages>,
	/// The TCGPlayer information for a given card.
	pub tcgplayer: Option<TCGPlayer>,
	/// The cardmarket information for a given card.
	pub cardmarket: Option<CardMarket>,
}

impl PartialEq for Card {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Card {}
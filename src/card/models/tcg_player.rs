use serde::{Deserialize, Serialize};

/// Holds the TCG Player metadata for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayer {
	/// The URL to the TCGPlayer store page to purchase this card.
	pub url: String,
	/// A date that the price was last updated. In the format of YYYY/MM/DD.
	pub updated_at: String,
	/// The available price data from TCGPlayer for each print. Prices are in USD.
	pub prices: Option<TCGPlayerPrints>,
}

/// Holds the TCG Player price data for each print style for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrints {
	/// Price information for the holofoil version of a card.
	pub holofoil: Option<TCGPlayerPrice>,
	/// Price information for the reverse holofoil version of a card.
	pub reverse_holo_foil: Option<TCGPlayerPrice>,
	/// Price information for the non-foil version of a card.
	pub normal: Option<TCGPlayerPrice>,
	/// Price information for the first edition holofoil version of a card.
	#[serde(rename = "1stEditionHolofoil")]
	pub first_edition_holofoil: Option<TCGPlayerPrice>,
	/// Price information for the first edition non-foil version of a card.
	#[serde(rename = "1stEditionNormal")]
	pub first_edition_normal: Option<TCGPlayerPrice>
}

/// Holds the TCG Player prices for a given print of a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrice {
	/// The low price of the card.
	pub low: Option<f64>,
	/// The mid price of the card.
	pub mid: Option<f64>,
	/// The high price of the card.
	pub high: Option<f64>,
	/// The market value of the card. This is usually the best representation of what people are willing to pay.
	pub market: Option<f64>,
	/// The direct low price of the card.
	pub direct_low: Option<f64>,
}
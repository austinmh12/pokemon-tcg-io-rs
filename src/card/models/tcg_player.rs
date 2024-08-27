use serde::{Deserialize, Serialize};

/// Holds the TCG Player metadata for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayer {
	pub url: String,
	pub updated_at: String,
	pub prices: Option<TCGPlayerPrints>,
}

/// Holds the TCG Player price data for each print style for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrints {
	pub holofoil: Option<TCGPlayerPrice>,
	pub reverse_holo_foil: Option<TCGPlayerPrice>,
	pub normal: Option<TCGPlayerPrice>,
}

/// Holds the TCG Player prices for a given print of a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrice {
	pub low: Option<f64>,
	pub mid: Option<f64>,
	pub high: Option<f64>,
	pub market: Option<f64>,
}
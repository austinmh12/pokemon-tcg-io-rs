use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayer {
	pub url: String,
	pub updated_at: String,
	pub prices: Option<TCGPlayerPrices>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrices {
	pub holofoil: Option<TCGPlayerPrice>,
	pub reverse_holo_foil: Option<TCGPlayerPrice>,
	pub normal: Option<TCGPlayerPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPlayerPrice {
	pub low: Option<f64>,
	pub mid: Option<f64>,
	pub high: Option<f64>,
	pub market: Option<f64>,
}
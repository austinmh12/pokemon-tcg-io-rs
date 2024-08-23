use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarket {
	pub url: String,
	pub updated_at: String,
	pub prices: Option<CardMarketPrices>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarketPrices {
	pub average_sell_price: Option<f64>,
	pub low_price: Option<f64>,
	pub trend_price: Option<f64>,
	pub german_pro_low: Option<f64>,
	pub suggested_price: Option<f64>,
	pub reverse_holo_sell: Option<f64>,
	pub reverse_holo_low: Option<f64>,
	pub reverse_holo_trend: Option<f64>,
	pub low_price_ex_plus: Option<f64>,
	pub avg1: Option<f64>,
	pub avg7: Option<f64>,
	pub avg30: Option<f64>,
	pub reverse_holo_avg1: Option<f64>,
	pub reverse_holo_avg7: Option<f64>,
	pub reverse_holo_avg30: Option<f64>,
}

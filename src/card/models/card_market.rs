use serde::{Deserialize, Serialize};

/// Holds the Card Market metadata for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarket {
	/// The URL to the cardmarket store page to purchase this card.
	pub url: String,
	/// A date that the price was last updated. In the format of YYYY/MM/DD.
	pub updated_at: String,
	/// All the prices provided by cardmarket in Euros.
	pub prices: Option<CardMarketPrices>,
}

/// Holds all of the Card Market price data. All prices are listed in Euros.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMarketPrices {
	/// The average sell price as shown in the chart at the website for non-foils.
	pub average_sell_price: Option<f64>,
	/// The lowest price at the market for non-foils.
	pub low_price: Option<f64>,
	/// The trend price as shown at the website (and in the chart) for non-foils.
	pub trend_price: Option<f64>,
	/// The lowest sell price from German professional sellers.
	pub german_pro_low: Option<f64>,
	/// A suggested sell price for professional users, determined by an internal algorithm; 
	/// this algorithm is controlled by cardmarket, not this API.
	pub suggested_price: Option<f64>,
	/// The average sell price as shown in the chart at the website for reverse holos.
	pub reverse_holo_sell: Option<f64>,
	/// The lowest price at the market as shown at the website (for condition EX+) for reverse holos.
	pub reverse_holo_low: Option<f64>,
	/// The trend price as shown at the website (and in the chart) for reverse holos.
	pub reverse_holo_trend: Option<f64>,
	/// The lowest price at the market for non-foils with condition EX or better.
	pub low_price_ex_plus: Option<f64>,
	/// The average sale price over the last day.
	pub avg1: Option<f64>,
	/// The average sale price over the last 7 days.
	pub avg7: Option<f64>,
	/// The average sale price over the last 30 days.
	pub avg30: Option<f64>,
	/// The average sale price over the last day for reverse holos.
	pub reverse_holo_avg1: Option<f64>,
	/// The average sale price over the last 7 days for reverse holos.
	pub reverse_holo_avg7: Option<f64>,
	/// The average sale price over the last 30 days for reverse holos.
	pub reverse_holo_avg30: Option<f64>,
}

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::SetImages;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
	pub id: String,
	pub name: String,
	pub series: String,
	pub printed_total: i64,
	pub total: i64,
	pub legalities: HashMap<String, String>,
	pub ptcgo_code: Option<String>,
	pub release_date: String,
	pub updated_at: String,
	pub images: SetImages,
}
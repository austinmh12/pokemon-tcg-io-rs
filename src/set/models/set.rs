use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::SetImages;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
	pub id: String,
	pub name: Option<String>,
	pub series: Option<String>,
	pub printed_total: Option<i64>,
	pub total: Option<i64>,
	pub legalities: Option<HashMap<String, String>>,
	pub ptcgo_code: Option<String>,
	pub release_date: Option<String>,
	pub updated_at: Option<String>,
	pub images: Option<SetImages>,
}

impl PartialEq for Set {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Set {}
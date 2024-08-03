#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PaginatedApiResponse<T> {
	pub(crate) data: Option<Vec<T>>,
	pub(crate) page: i32,
	pub(crate) page_size: i32,
	pub(crate) count: i32,
	pub(crate) total_count: i32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiResponse<T> {
	pub(crate) data: Option<T>,
}
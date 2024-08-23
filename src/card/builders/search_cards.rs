use crate::{Client, Result};
use crate::client::PaginatedApiResponse;
use crate::Card;

#[derive(Debug, Clone)]
pub struct SearchCardsBuilder {
	client: Client,
	request: SearchCards,
}

#[derive(Debug, Clone, Default)]
pub struct SearchCards {
	query: Option<String>,
	page: Option<u32>,
	page_size: Option<u32>,
	order_by: Option<String>,
	select: Option<String>,
}

impl SearchCardsBuilder {
	pub(crate) fn new(client: Client) -> SearchCardsBuilder {
		SearchCardsBuilder { client, request: SearchCards::default() }
	}

	pub fn query(mut self, q: impl Into<String>) -> SearchCardsBuilder {
		self.request.query = Some(q.into());
		self
	}

	pub async fn send(self) -> Result<PaginatedApiResponse<Card>> {
		let mut params = vec![];
		if let Some(q) = &self.request.query {
			params.push(("q", q.as_str()));
		}
		self.client.get::<PaginatedApiResponse<Card>>("cards", Some(params)).await
	}
}
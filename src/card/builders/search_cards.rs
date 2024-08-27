use std::borrow::Cow;
use std::future::IntoFuture;

use crate::{Client, Requestable, Result};
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

impl Requestable for SearchCards {
	fn endpoint(&self) -> Cow<'static, str> {
		"cards".into()
	}

	fn params(&self) -> Vec<(String, String)> {
		let mut ret = vec![];
		if let Some(q) = &self.query {
			ret.push(("q".to_string(), q.clone()));
		}
		if let Some(page) = &self.page {
			ret.push(("page".to_string(), page.to_string()));
		}
		if let Some(page_size) = &self.page_size {
			ret.push(("pageSize".to_string(), page_size.to_string()));
		}
		if let Some(order) = &self.order_by {
			ret.push(("orderBy".to_string(), order.clone()));
		}
		if let Some(select) = &self.select {
			ret.push(("select".to_string(), select.clone()));
		}

		ret
	}
}

impl SearchCardsBuilder {
	pub(crate) fn new(client: Client) -> SearchCardsBuilder {
		SearchCardsBuilder { client, request: SearchCards::default() }
	}

	pub fn query(mut self, value: impl Into<String>) -> SearchCardsBuilder {
		self.request.query = Some(value.into());
		self
	}

	pub fn page(mut self, value: u32) -> SearchCardsBuilder {
		self.request.page = Some(value);
		self
	}

	pub fn page_size(mut self, value: u32) -> SearchCardsBuilder {
		self.request.page_size = Some(value);
		self
	}

	pub fn order_by(mut self, value: impl Into<String>) -> SearchCardsBuilder {
		self.request.order_by = Some(value.into());
		self
	}

	pub fn select(mut self, value: impl Into<String>) -> SearchCardsBuilder {
		let mut val: String = value.into();
		if !val.contains("id") {
			val.push_str(",id");
		}
		self.request.select = Some(val);
		self
	}

	pub async fn send(self) -> Result<Option<Vec<Card>>> {
		let mut cards: Vec<Card> = vec![];
		let mut request = self.request.clone();
		// Get all pages if none is specified
		if request.page.is_none() {
			loop {
				let ret: PaginatedApiResponse<Card> = self.client.get(request.clone()).await?;
				cards.extend(ret.data.unwrap_or_default());
				if ret.page >= ret.total_count / ret.page_size {
					break;
				}
				*request.page.get_or_insert(1) += 1;
			}
		// Otherwise fetch the specified page
		} else {
			let ret: PaginatedApiResponse<Card> = self.client.get(request.clone()).await?;
			cards.extend(ret.data.unwrap_or_default());
		}
		

		if cards.len() > 0usize {
			Ok(Some(cards))
		} else {
			Ok(None)
		}
	}
}

impl IntoFuture for SearchCardsBuilder {
	type Output = Result<Option<Vec<Card>>>;
	type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(self.send())
	}
}

// Client implementations
impl Client {
	pub fn search_cards(&self) -> SearchCardsBuilder {
		SearchCardsBuilder::new(self.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	fn client() -> Client {
		let poketcg_key = dotenv::var("POKETCGAPIKEY").unwrap();
		Client::builder().api_key(poketcg_key).build()
	}

	#[tokio::test]
	async fn test_search_cards() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().send().await?;
		assert!(searched_cards.is_some());
		let cards = searched_cards.unwrap();
		assert!(cards.len() > 250usize);

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_query() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().query("name:magikarp").send().await?;
		assert!(searched_cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_page() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().page(2).send().await?;
		assert!(searched_cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_page_size() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().page(1).page_size(2).send().await?;
		assert!(searched_cards.is_some());
		assert_eq!(searched_cards.unwrap().len(), 2usize);

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_order_by() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().page(1).order_by("number").send().await?;
		assert!(searched_cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_select() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().page(1).select("id").send().await?;
		assert!(searched_cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_with_select_without_required_fields() -> Result<()> {
		let client = client();
		let searched_cards = client.search_cards().page(1).select("number").send().await?;
		assert!(searched_cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_await() -> Result<()> {
		let client = client();
		let cards = client.search_cards().page(1).await?;
		assert!(cards.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_cards_builder_await() -> Result<()> {
		let client = client();
		let cards = client.search_cards()
			.page(1)
			.page_size(10)
			.query("name:magikarp")
			.order_by("number")
			.select("id,number,name")
			.await?;
		assert!(cards.is_some());
		let cards = cards.unwrap();
		assert_eq!(cards.len(), 10usize);

		Ok(())
	}
}
use std::sync::Arc;
use super::{PaginatedApiResponse, ApiResponse};
use crate::{models::Card, Error, Result};

#[derive(Debug, Clone)]
pub struct Client {
	pub(super) inner: Arc<ClientInner>
}

impl Default for Client {
	fn default() -> Self {
		Client::builder().build()
	}
}

impl Client {
	pub fn builder() -> ClientBuilder {
		ClientBuilder::default()
	}

	fn web_client(&self) -> &reqwest::Client {
		&self.inner.web_client
	}

	fn api_key(&self) -> String {
		self.inner.api_key.as_ref().unwrap_or(&String::from("")).to_string()
	}

	async fn get<T>(&self, endpoint: &str, params: Option<Vec<(&str, &str)>>) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
	{
		let mut req = self.web_client()
			.get(format!("https://api.pokemontcg.io/v2/{}", endpoint))
			.header("X-Api-Key", self.api_key());
		if let Some(params) = params {
			req = req.query(&params);
		}
		// let tmp = req.try_clone().unwrap().send().await.unwrap();
		// let txt = tmp.text().await.unwrap();
		// println!("{}", txt);
		// let ret: T = req.send().await.map_err(|_| Error::Something)?.json().await.map_err(|_| Error::Something)?;
		let ret: T = req.send().await.unwrap().json().await.unwrap();
		Ok(ret)
	}

	pub async fn get_cards(&self) -> Result<Option<Vec<Card>>> {
		let resp: PaginatedApiResponse<Card> = self.get("cards", None).await?;
		Ok(resp.data)
	}

	pub async fn get_card(&self, id: impl Into<String>) -> Result<Option<Card>> {
		let resp: ApiResponse<Card> = self.get(&format!("cards/{}", id.into()), None).await?;
		Ok(resp.data)
	}

	pub async fn search_cards(&self, q: Option<&str>) -> Result<Option<Vec<Card>>> {
		let params = if let Some(query) = q {
			Some(vec![("q", query)])
		} else {
			None
		};
		let resp: PaginatedApiResponse<Card> = self.get("cards", params).await?;
		Ok(resp.data)
	}
}


/// ClientInner
#[derive(Debug)]
pub(super) struct ClientInner {
	pub(super) web_client: reqwest::Client,
	pub(super) api_key: Option<String>
}

impl Default for ClientInner {
	fn default() -> Self {
		Self {
			web_client: reqwest::Client::new(),
			api_key: None
		}
	}
}

/// ClientBuilder
#[derive(Debug, Default)]
pub struct ClientBuilder {
	web_client: Option<reqwest::Client>,
	api_key: Option<String>
}

/// Builder methods
impl ClientBuilder {
	pub fn with_reqwest(mut self, reqwest_client: reqwest::Client) -> Self {
		self.web_client = Some(reqwest_client);
		self
	}

	pub fn api_key(mut self, key: impl Into<String>) -> Self {
		self.api_key = Some(key.into());
		self
	}
}

/// Build() method
impl ClientBuilder {
	pub fn build(self) -> Client {
		let inner = ClientInner {
			web_client: self.web_client.unwrap_or(reqwest::Client::new()),
			api_key: self.api_key
		};
		Client { inner: Arc::new(inner) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[tokio::test]
	#[ignore] // Heavy test, doesn't need to be run by default
	async fn test_cards() -> Result<()> {
		let poketcg_key = dotenv::var("POKETCGAPIKEY").unwrap();
		let client = Client::builder().api_key(poketcg_key).build();
		let cards = client.get_cards().await?;
		assert!(cards.is_some());
		Ok(())
	}

	#[tokio::test]
	async fn test_card() -> Result<()> {
		let poketcg_key = dotenv::var("POKETCGAPIKEY").unwrap();
		let client = Client::builder().api_key(poketcg_key).build();
		let card = client.get_card("xy1-1").await?;
		assert!(card.is_some());
		assert_eq!(card.unwrap().id, String::from("xy1-1"));
		Ok(())
	}

	async fn test_card_with_select() {
		todo!()
	}

	// #[tokio::test]
	async fn test_search_cards() -> Result<()> {
		todo!()
	}

	async fn test_search_cards_with_page() -> Result<()> {
		todo!()
	}

	async fn test_search_cards_with_page_size() -> Result<()> {
		todo!()
	}

	async fn test_search_cards_with_order_by() -> Result<()> {
		todo!()
	}

	async fn test_search_cards_with_select() -> Result<()> {
		todo!()
	}

	async fn test_get_sets() -> Result<()> {
		todo!()
	}

	async fn test_get_set() -> Result<()> {
		todo!()
	}

	async fn test_get_set_with_select() -> Result<()> {
		todo!()
	}

	async fn test_search_sets() -> Result<()> {
		todo!()
	}

	async fn test_search_sets_with_page() -> Result<()> {
		todo!()
	}

	async fn test_search_sets_with_page_size() -> Result<()> {
		todo!()
	}

	async fn test_search_sets_with_order_by() -> Result<()> {
		todo!()
	}

	async fn test_search_sets_with_select() -> Result<()> {
		todo!()
	}

	async fn test_get_types() -> Result<()> {
		todo!()
	}

	async fn test_get_subtypes() -> Result<()> {
		todo!()
	}

	async fn test_get_rarities() -> Result<()> {
		todo!()
	}
}
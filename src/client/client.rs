use std::sync::Arc;
use super::{PaginatedApiResponse, ApiResponse};
use crate::{Card, Error, Requestable, Result};
use crate::card::*;

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
	// What I want client.get_card() to return is a GetCard builder that has a select() method and a send() method
	// So that you can call client.get_card("id").send().await?; to get the card
	// Or client.get_card("id").select("name").send().await?; to get the card with just the name filled out
	// Heavily inspired by reqwest and it's client.get() -> RequestBuilder / client.get().send() api
	pub fn builder() -> ClientBuilder {
		ClientBuilder::default()
	}

	fn web_client(&self) -> &reqwest::Client {
		&self.inner.web_client
	}

	fn api_key(&self) -> String {
		self.inner.api_key.as_ref().unwrap_or(&String::from("")).to_string()
	}

	pub(crate) async fn get<T, R>(&self, request: R) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
		R: Requestable,
	{
		let req = self.web_client()
			.get(format!("https://api.pokemontcg.io/v2/{}", request.endpoint()))
			.header("X-Api-Key", self.api_key())
			.query(&request.params());
		println!("{:?}", &req);
		// let tmp = req.try_clone().unwrap().send().await.unwrap();
		// let txt = tmp.text().await.unwrap();
		// println!("{}", txt);
		// let ret: T = req.send().await.map_err(|_| Error::Something)?.json().await.map_err(|_| Error::Something)?;
		let ret: T = req.send().await.unwrap().json().await.unwrap();
		Ok(ret)
	}

	// pub async fn get_cards(&self) -> Result<Option<Vec<Card>>> {
	// 	let resp: PaginatedApiResponse<Card> = self.get("cards", None).await?;
	// 	Ok(resp.data)
	// }

	// pub async fn get_card(&self, id: impl Into<String>) -> Result<Option<Card>> {
	// 	let resp: ApiResponse<Card> = self.get(&format!("cards/{}", id.into()), None).await?;
	// 	Ok(resp.data)
	// }

	// pub async fn search_cards(&self, q: Option<&str>) -> Result<Option<Vec<Card>>> {
	// 	let params = if let Some(query) = q {
	// 		Some(vec![("q", query)])
	// 	} else {
	// 		None
	// 	};
	// 	let resp: PaginatedApiResponse<Card> = self.get("cards", params).await?;
	// 	Ok(resp.data)
	// }
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
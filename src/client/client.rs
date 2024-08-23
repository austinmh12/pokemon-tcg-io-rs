use std::sync::Arc;
use crate::{Requestable, Result};

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
		let ret: T = req.send().await.unwrap().json().await.unwrap();
		Ok(ret)
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
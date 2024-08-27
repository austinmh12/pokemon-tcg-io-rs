use std::sync::Arc;
use crate::{Requestable, Result};

/// A client for the Pokémon TCG IO REST API.
#[derive(Debug, Clone)]
pub struct Client {
	inner: Arc<ClientInner>
}

impl Default for Client {
	fn default() -> Self {
		Client::builder().build()
	}
}

impl Client {
	/// Provides a `ClientBuilder` for configuring the `Client` instance.
	pub fn builder() -> ClientBuilder {
		ClientBuilder::default()
	}

	/// Returns a `Client` with the API key set.
	pub fn with_api_key(key: impl Into<String>) -> Client {
		ClientBuilder::default().api_key(key).build()
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

/// A builder to construct the properties for a `Client`.
#[derive(Debug, Default)]
pub struct ClientBuilder {
	web_client: Option<reqwest::Client>,
	api_key: Option<String>
}

/// Builder methods
impl ClientBuilder {
	/// Provide a custom reqwest client to the `Client`.
	pub fn with_reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
		self.web_client = Some(reqwest_client);
		self
	}

	/// Provide an API key to the client.
	pub fn api_key(mut self, key: impl Into<String>) -> Self {
		self.api_key = Some(key.into());
		self
	}
}

impl ClientBuilder {
	/// Build the `Client` struct with the builder's configuration.
	pub fn build(self) -> Client {
		let inner = ClientInner {
			web_client: self.web_client.unwrap_or(reqwest::Client::new()),
			api_key: self.api_key
		};
		Client { inner: Arc::new(inner) }
	}
}
use std::borrow::Cow;
use std::future::IntoFuture;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

/// A builder to construct the properties for the rarities endpoint
/// 
/// To construct a `GetRaritiesBuilder`, refer to the `Client` documentation.
#[derive(Debug, Clone)]
pub struct GetRaritiesBuilder {
	client: Client,
	request: GetRarities,
}

#[derive(Debug, Clone, Default)]
pub struct GetRarities;

impl Requestable for GetRarities {
	fn endpoint(&self) -> Cow<'static, str> {
		"rarities".into()
	}
}

impl GetRaritiesBuilder {
	pub(crate) fn new(client: Client) -> GetRaritiesBuilder {
		GetRaritiesBuilder { client, request: GetRarities::default() }
	}

	/// Sends the request to the rarities endpoint with the provided parameters.
	/// 
	/// This is called when awaiting the `GetRaritiesBuilder` as well.
	/// 
	/// # Errors
	/// 
	/// This method fails if there was an error sending the request or if the response
	/// doesn't include a field due to an error in the API.
	/// 
	/// # Example
	/// 
	/// ```no_run
	/// # use pokemontcgio::{Client, Result};
	/// # 
	/// # async fn run() -> Result<()> {
	/// let client = Client::with_api_key("YOUR_KEY");
	/// client.get_rarities().send().await?;
	/// // or
	/// client.get_rarities().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

impl IntoFuture for GetRaritiesBuilder {
	type Output = Result<Option<Vec<String>>>;
	type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(self.send())
	}
}

// Client implementations
impl Client {
	/// Convenience method to make a request to the rarities endpoint.
	pub fn get_rarities(&self) -> GetRaritiesBuilder {
		GetRaritiesBuilder::new(self.clone())
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
	async fn test_get_rarities() -> Result<()> {
		let client = client();
		let rarities = client.get_rarities().send().await?;
		assert!(rarities.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_get_rarities_await() -> Result<()> {
		let client = client();
		let rarities = client.get_rarities().await?;
		assert!(rarities.is_some());

		Ok(())
	}
}
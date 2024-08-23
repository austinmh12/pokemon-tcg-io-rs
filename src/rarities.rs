use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

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

	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

// Client implementations
impl Client {
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
}
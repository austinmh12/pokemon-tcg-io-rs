use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

#[derive(Debug, Clone)]
pub struct GetTypesBuilder {
	client: Client,
	request: GetTypes,
}

#[derive(Debug, Clone, Default)]
pub struct GetTypes;

impl Requestable for GetTypes {
	fn endpoint(&self) -> Cow<'static, str> {
		"types".into()
	}
}

impl GetTypesBuilder {
	pub(crate) fn new(client: Client) -> GetTypesBuilder {
		GetTypesBuilder { client, request: GetTypes::default() }
	}

	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

// Client implementations
impl Client {
	pub fn get_types(&self) -> GetTypesBuilder {
		GetTypesBuilder::new(self.clone())
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
	async fn test_get_types() -> Result<()> {
		let client = client();
		let types = client.get_types().send().await?;
		assert!(types.is_some());

		Ok(())
	}
}
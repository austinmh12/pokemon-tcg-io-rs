use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

#[derive(Debug, Clone)]
pub struct GetSubtypesBuilder {
	client: Client,
	request: GetSubtypes,
}

#[derive(Debug, Clone, Default)]
pub struct GetSubtypes;

impl Requestable for GetSubtypes {
	fn endpoint(&self) -> Cow<'static, str> {
		"subtypes".into()
	}
}

impl GetSubtypesBuilder {
	pub(crate) fn new(client: Client) -> GetSubtypesBuilder {
		GetSubtypesBuilder { client, request: GetSubtypes::default() }
	}

	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

// Client implementations
impl Client {
	pub fn get_subtypes(&self) -> GetSubtypesBuilder {
		GetSubtypesBuilder::new(self.clone())
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
	async fn test_get_subtypes() -> Result<()> {
		let client = client();
		let subtypes = client.get_subtypes().send().await?;
		assert!(subtypes.is_some());

		Ok(())
	}
}
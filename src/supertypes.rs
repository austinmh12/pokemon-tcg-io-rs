use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

#[derive(Debug, Clone)]
pub struct GetSupertypesBuilder {
	client: Client,
	request: GetSupertypes,
}

#[derive(Debug, Clone, Default)]
pub struct GetSupertypes;

impl Requestable for GetSupertypes {
	fn endpoint(&self) -> Cow<'static, str> {
		"supertypes".into()
	}
}

impl GetSupertypesBuilder {
	pub(crate) fn new(client: Client) -> GetSupertypesBuilder {
		GetSupertypesBuilder { client, request: GetSupertypes::default() }
	}

	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

// Client implementations
impl Client {
	pub fn get_supertypes(&self) -> GetSupertypesBuilder {
		GetSupertypesBuilder::new(self.clone())
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
	async fn test_get_supertypes() -> Result<()> {
		let client = client();
		let supertypes = client.get_supertypes().send().await?;
		assert!(supertypes.is_some());

		Ok(())
	}
}
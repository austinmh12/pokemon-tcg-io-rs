use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;

/// A builder to construct the properties for the types endpoint
/// 
/// To construct a `GetTypesBuilder`, refer to the `Client` documentation.
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

	/// Sends the request to the types endpoint with the provided parameters.
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
	/// client.get_types("base1").send().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

// Client implementations
impl Client {
	/// Convenience method to make a request to the types endpoint.
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
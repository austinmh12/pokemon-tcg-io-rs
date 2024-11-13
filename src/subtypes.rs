use std::borrow::Cow;
use std::future::IntoFuture;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;
use crate::utils::futurize;

/// A builder to construct the properties for the subtypes endpoint
/// 
/// To construct a `GetSubtypesBuilder`, refer to the `Client` documentation.
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

	/// Sends the request to the subtypes endpoint with the provided parameters.
	/// 
	/// This is called when awaiting the `GetSubtypesBuilder` as well.
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
	/// client.get_subtypes().await?;
	/// # Ok(())
	/// # }
	/// ```
	async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

futurize!(GetSubtypesBuilder, Option<Vec<String>>);

// Client implementations
impl Client {
	/// Convenience method to make a request to the subtypes endpoint.
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
		let subtypes = client.get_subtypes().await?;
		assert!(subtypes.is_some());

		Ok(())
	}
}
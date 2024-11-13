use std::borrow::Cow;
use std::future::IntoFuture;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;
use crate::utils::futurize;

/// A builder to construct the properties for the supertypes endpoint
/// 
/// To construct a `GetSupertypesBuilder`, refer to the `Client` documentation.
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

	/// Sends the request to the supertypes endpoint with the provided parameters.
	/// 
	/// This is called when awaiting the `GetSupertypesBuilder` as well.
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
	/// client.get_supertypes().await?;
	/// # Ok(())
	/// # }
	/// ```
	async fn send(self) -> Result<Option<Vec<String>>> {
		let ret: ApiResponse<Vec<String>> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

futurize!(GetSupertypesBuilder, Option<Vec<String>>);

// Client implementations
impl Client {
	/// Convenience method to make a request to the supertypes endpoint.
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
		let supertypes = client.get_supertypes().await?;
		assert!(supertypes.is_some());

		Ok(())
	}
}
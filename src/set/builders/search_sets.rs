use std::borrow::Cow;
use std::future::IntoFuture;

use crate::{Client, Requestable, Result};
use crate::client::PaginatedApiResponse;
use crate::Set;

/// A builder to construct the properties for the sets endpoint
/// 
/// To construct a `SearchSetsBuilder`, refer to the `Client` documentation.
#[derive(Debug, Clone)]
pub struct SearchSetsBuilder {
	client: Client,
	request: SearchSets,
}

#[derive(Debug, Clone, Default)]
pub struct SearchSets {
	query: Option<String>,
	page: Option<u32>,
	page_size: Option<u32>,
	order_by: Option<String>,
	select: Option<String>,
}

impl Requestable for SearchSets {
	fn endpoint(&self) -> Cow<'static, str> {
		"sets".into()
	}

	fn params(&self) -> Vec<(String, String)> {
		let mut ret = vec![];
		if let Some(q) = &self.query {
			ret.push(("q".to_string(), q.clone()));
		}
		if let Some(page) = &self.page {
			ret.push(("page".to_string(), page.to_string()));
		}
		if let Some(page_size) = &self.page_size {
			ret.push(("pageSize".to_string(), page_size.to_string()));
		}
		if let Some(order) = &self.order_by {
			ret.push(("orderBy".to_string(), order.clone()));
		}
		if let Some(select) = &self.select {
			ret.push(("select".to_string(), select.clone()));
		}

		ret
	}
}

impl SearchSetsBuilder {
	pub(crate) fn new(client: Client) -> SearchSetsBuilder {
		SearchSetsBuilder { client, request: SearchSets::default() }
	}

	/// Query to search with.
	/// 
	/// For information on the syntax, go to <https://pokemontcg.guru/syntax>
	pub fn query(mut self, value: impl Into<String>) -> SearchSetsBuilder {
		self.request.query = Some(value.into());
		self
	}

	/// Page to start fetching results from.
	/// 
	/// If not provided, all results are fetched.
	pub fn page(mut self, value: u32) -> SearchSetsBuilder {
		self.request.page = Some(value);
		self
	}

	/// The size of the results.
	pub fn page_size(mut self, value: u32) -> SearchSetsBuilder {
		self.request.page_size = Some(value);
		self
	}

	/// Order of the results.
	pub fn order_by(mut self, value: impl Into<String>) -> SearchSetsBuilder {
		self.request.order_by = Some(value.into());
		self
	}

	/// Specific fields to fetch with the sets.
	/// 
	/// Always includes "id" if not added.
	pub fn select(mut self, value: impl Into<String>) -> SearchSetsBuilder {
		let mut val: String = value.into();
		if !val.contains("id") {
			val.push_str(",id");
		}
		self.request.select = Some(val);
		self
	}

	/// Sends the request to the sets endpoint with the provided parameters.
	/// 
	/// This is called when awaiting the `SearchSetsBuilder` as well.
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
	/// client.search_sets().send().await?;
	/// // or
	/// client.search_sets().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn send(self) -> Result<Option<Vec<Set>>> {
		let mut sets: Vec<Set> = vec![];
		let mut request = self.request.clone();
		// Get all pages if none is specified
		if request.page.is_none() {
			loop {
				let ret: PaginatedApiResponse<Set> = self.client.get(request.clone()).await?;
				sets.extend(ret.data.unwrap_or_default());
				if ret.page >= ret.total_count / ret.page_size {
					break;
				}
				*request.page.get_or_insert(1) += 1;
			}
		// Otherwise fetch the specified page
		} else {
			let ret: PaginatedApiResponse<Set> = self.client.get(request.clone()).await?;
			sets.extend(ret.data.unwrap_or_default());
		}
		

		if sets.len() > 0usize {
			Ok(Some(sets))
		} else {
			Ok(None)
		}
	}
}

impl IntoFuture for SearchSetsBuilder {
	type Output = Result<Option<Vec<Set>>>;
	type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(self.send())
	}
}

// Client implementations
impl Client {
	/// Convenience method to make a request to the sets endpoint.
	pub fn search_sets(&self) -> SearchSetsBuilder {
		SearchSetsBuilder::new(self.clone())
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
	async fn test_search_sets() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	#[ignore]
	async fn test_search_sets_with_query() -> Result<()> {
		let client = client();
		// This endpoint doesn't like reqwests serialization "name%3ASword+%26+Shield" vs what it wants "name:Sword%20&%20Shield"
		let searched_sets = client.search_sets().query("name:Sword & Shield").send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_with_page() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().page(2).page_size(1).send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_with_page_size() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().page(1).page_size(2).send().await?;
		assert!(searched_sets.is_some());
		assert_eq!(searched_sets.unwrap().len(), 2usize);

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_with_order_by() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().order_by("releaseDate").send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_with_select() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().select("id").send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_with_select_without_required_fields() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().select("total").send().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}

	#[tokio::test]
	async fn test_search_sets_await() -> Result<()> {
		let client = client();
		let searched_sets = client.search_sets().await?;
		assert!(searched_sets.is_some());

		Ok(())
	}
}
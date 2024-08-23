use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;
use crate::Set;

#[derive(Debug, Clone)]
pub struct GetSetBuilder {
	client: Client,
	request: GetSet,
}

#[derive(Debug, Clone)]
pub struct GetSet {
	id: String,
	select: Option<String>,
}

impl GetSet {
	fn new(id: String) -> GetSet {
		GetSet { id, select: None }
	}
}

impl Requestable for GetSet {
	fn endpoint(&self) -> Cow<'static, str> {
		format!("sets/{}", &self.id).into()
	}

	fn params(&self) -> Vec<(String, String)> {
		let mut ret = vec![];
		if let Some(select) = &self.select {
			ret.push(("select".to_string(), select.clone()));
		}

		ret
	}
}

impl GetSetBuilder {
	pub(crate) fn new(client: Client, id: impl Into<String>) -> GetSetBuilder {
		GetSetBuilder { client, request: GetSet::new(id.into()) }
	}

	pub fn select(mut self, value: impl Into<String>) -> GetSetBuilder {
		let mut val: String = value.into();
		if !val.contains("id") {
			val.push_str(",id");
		}
		self.request.select = Some(val);
		self
	}

	pub async fn send(self) -> Result<Option<Set>> {
		let ret: ApiResponse<Set> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

impl Client {
	pub fn get_set(&self, id: impl Into<String>) -> GetSetBuilder {
		GetSetBuilder::new(self.clone(), id)
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
	async fn test_set() -> Result<()> {
		let client = client();
		let set = client.get_set("swsh1").send().await?;
		assert!(set.is_some());
		let set = set.unwrap();
		assert_eq!(set.id, String::from("swsh1"));
		Ok(())
	}

	#[tokio::test]
	async fn test_set_with_select() -> Result<()> {
		let client = client();
		let set = client.get_set("swsh1").select("id").send().await?;
		assert!(set.is_some());
		let set = set.unwrap();
		assert_eq!(set.id, String::from("swsh1"));
		assert_eq!(set.name, None);

		Ok(())
	}

	#[tokio::test]
	async fn test_set_with_select_without_required_fields() -> Result<()> {
		let client = client();
		let set = client.get_set("swsh1").select("total").send().await?;
		assert!(set.is_some());
		let set = set.unwrap();
		assert_eq!(set.id, String::from("swsh1"));
		assert_eq!(set.name, None);
		assert!(set.total.is_some());

		Ok(())
	}
}
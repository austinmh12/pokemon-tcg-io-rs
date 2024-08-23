use std::borrow::Cow;

use crate::{Client, Requestable, Result};
use crate::client::ApiResponse;
use crate::Card;

#[derive(Debug, Clone)]
pub struct GetCardBuilder {
	client: Client,
	request: GetCard,
}

#[derive(Debug, Clone)]
pub struct GetCard {
	id: String,
	select: Option<String>,
}

impl GetCard {
	fn new(id: String) -> GetCard {
		GetCard { id, select: None }
	}
}

impl Requestable for GetCard {
	fn endpoint(&self) -> Cow<'static, str> {
		format!("cards/{}", &self.id).into()
	}

	fn params(&self) -> Vec<(String, String)> {
		let mut ret = vec![];
		if let Some(select) = &self.select {
			ret.push(("select".to_string(), select.clone()));
		}

		ret
	}
}

impl GetCardBuilder {
	pub(crate) fn new(client: Client, id: impl Into<String>) -> GetCardBuilder {
		GetCardBuilder { client, request: GetCard::new(id.into()) }
	}

	pub fn select(mut self, value: impl Into<String>) -> GetCardBuilder {
		let mut val: String = value.into();
		if !val.contains("id") {
			val.push_str(",id");
		}
		self.request.select = Some(val);
		self
	}

	pub async fn send(self) -> Result<Option<Card>> {
		let ret: ApiResponse<Card> = self.client.get(self.request).await?;
		Ok(ret.data)
	}
}

impl Client {
	pub fn get_card(&self, id: impl Into<String>) -> GetCardBuilder {
		GetCardBuilder::new(self.clone(), id)
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
	async fn test_card() -> Result<()> {
		let client = client();
		let card = client.get_card("xy1-1").send().await?;
		assert!(card.is_some());
		let card = card.unwrap();
		assert_eq!(card.id, String::from("xy1-1"));
		Ok(())
	}

	#[tokio::test]
	async fn test_card_with_select() -> Result<()> {
		let client = client();
		let card = client.get_card("xy1-1").select("id").send().await?;
		assert!(card.is_some());
		let card = card.unwrap();
		assert_eq!(card.id, String::from("xy1-1"));
		assert_eq!(card.name, None);

		Ok(())
	}

	#[tokio::test]
	async fn test_card_with_select_without_required_fields() -> Result<()> {
		let client = client();
		let card = client.get_card("xy1-1").select("number,set").send().await?;
		assert!(card.is_some());
		let card = card.unwrap();
		assert_eq!(card.id, String::from("xy1-1"));
		assert_eq!(card.name, None);
		assert!(card.set.is_some());

		Ok(())
	}
}
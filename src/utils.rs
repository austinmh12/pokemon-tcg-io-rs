use std::borrow::Cow;

pub trait Requestable {
	fn endpoint(&self) -> Cow<'static, str>;

	fn params(&self) -> Vec<(String, String)> {
		vec![]
	}
}
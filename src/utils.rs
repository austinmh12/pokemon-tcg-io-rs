use std::borrow::Cow;

pub trait Requestable {
	fn endpoint(&self) -> Cow<'static, str>;

	fn params(&self) -> Vec<(String, String)> {
		vec![]
	}
}

macro_rules! futurize {
	($struct:ty, $out:ty) => {
		impl IntoFuture for $struct {
			type Output = Result<$out>;
			type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output>>>;
		
			fn into_future(self) -> Self::IntoFuture {
				Box::pin(self.send())
			}
		}
	};
}

pub(crate) use futurize;
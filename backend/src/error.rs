use super::*;

#[ derive (Debug, thiserror::Error) ]
pub enum ErrorResponse {
	#[ error ("{0}") ]
	Anyhow (#[ from ] anyhow::Error),
	#[ error ("{1}") ]
	Http (HttpStatus, anyhow::Error),
}

impl ErrorResponse {
	pub fn forbidden (inner: anyhow::Error) -> Self {
		Self::Http (HttpStatus::FORBIDDEN, inner)
	}
	pub fn unauthorized (inner: anyhow::Error) -> Self {
		Self::Http (HttpStatus::UNAUTHORIZED, inner)
	}
}

impl From <reqwest::Error> for ErrorResponse {
	fn from (err: reqwest::Error) -> Self {
		Self::from (anyhow::Error::from (err))
	}
}

impl IntoResponse for ErrorResponse {
	fn into_response (self) -> Response <Body> {
		match self {
			Self::Anyhow (err) =>
				Response::builder ()
					.status (HttpStatus::INTERNAL_SERVER_ERROR)
					.body (Body::new (format! ("{err}\n")))
					.unwrap (),
			Self::Http (status, err) =>
				Response::builder ()
					.status (status)
					.body (Body::new (format! ("{err}\n")))
					.unwrap (),
		}
	}
}

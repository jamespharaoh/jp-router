pub fn url_encode_path <'dat> (val: & 'dat str) -> percent_encoding::PercentEncode <'dat> {
	percent_encoding::utf8_percent_encode (val, percent_encoding::NON_ALPHANUMERIC)
}

pub mod ex {
	pub use axum::extract::Path;
	pub use axum::extract::State;
	pub use axum_extra::TypedHeader;
	pub use headers::Authorization;
	pub use headers::authorization::Basic;
	pub type AuthBasic =
		axum_extra::TypedHeader <headers::Authorization <headers::authorization::Basic>>;
}

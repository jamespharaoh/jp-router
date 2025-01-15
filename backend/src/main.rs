use arcstr::ArcStr;

use axum::Router;

use axum::Json;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing as r;

use chrono::TimeZone;
use chrono::Utc;

use google_cloud_token::TokenSourceProvider as _;

use http::status::StatusCode as HttpStatus;

use serde::Deserialize;
use serde::Serialize;

use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;
use std::sync::Arc;

use jp_router_common::*;

mod acme_verify;
mod config;
mod dhcp_leases;
mod google;

use config::*;
use google::*;

#[ tokio::main ]
async fn main () -> anyhow::Result <()> {

	eprintln! ("Loading config");
	let config = load_config ().await ?;

	eprintln! ("Obtaining Google Cloud credentials");
	let google_auth = GoogleAuth::build (& config).await ?;

	eprintln! ("Starting web server on {}", config.core.listen);
	let state = Arc::new (GlobalState {
		config: config.clone (),
		google_auth,
		http: reqwest::Client::new (),
	});
	let app =
		axum::Router::new ()
			.nest ("/acme-verify", acme_verify::router ())
			.nest ("/dhcp-leases", dhcp_leases::router ())
			.layer (tower::ServiceBuilder::new ()
				.layer (tower_http::cors::CorsLayer::new ()
					.allow_methods ([ http::Method::GET, http::Method::POST ])
					.allow_origin (tower_http::cors::Any)))
			.with_state (state.clone ());

	let listener =
		tokio::net::TcpListener::bind (& * state.config.core.listen)
			.await.unwrap ();
	axum::serve (listener, app).await ?;

	Ok (())

}

struct GlobalState {
	config: Arc <Config>,
	google_auth: GoogleAuth,
	http: reqwest::Client,
}

#[ derive (Debug, thiserror::Error) ]
enum ErrorResponse {
	#[ error ("{0}") ]
	Anyhow (#[ from ] anyhow::Error),
	#[ error ("{1}") ]
	Http (HttpStatus, anyhow::Error),
}

impl ErrorResponse {
	fn forbidden (inner: anyhow::Error) -> Self {
		Self::Http (HttpStatus::FORBIDDEN, inner)
	}
	fn internal (inner: anyhow::Error) -> Self {
		Self::Http (HttpStatus::INTERNAL_SERVER_ERROR, inner)
	}
	fn unauthorized (inner: anyhow::Error) -> Self {
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

fn url_encode <'dat> (val: & 'dat str) -> percent_encoding::PercentEncode <'dat> {
	percent_encoding::utf8_percent_encode (val, percent_encoding::NON_ALPHANUMERIC)
}

mod ex {
	pub use axum::extract::Path;
	pub use axum::extract::State;
	pub use axum_extra::TypedHeader;
	pub use headers::Authorization;
	pub use headers::authorization::Basic;
	pub type AuthBasic =
		axum_extra::TypedHeader <headers::Authorization <headers::authorization::Basic>>;
}

use arcstr::ArcStr;

use axum::Router;

use axum::Json;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing as r;

use chrono::TimeZone;
use chrono::Utc;

use futures::prelude::*;

use google_cloud_token::TokenSourceProvider as _;

use http::status::StatusCode as HttpStatus;

use itertools::Itertools;

use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use jp_router_common::*;

mod acme_verify;
mod config;
mod error;
mod dhcp_leases;
mod google;
mod misc;
mod networks;

use config::*;
use error::*;
use google::auth::GoogleAuth;
use misc::*;

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
			.nest ("/networks", networks::router ())
			.layer (tower::ServiceBuilder::new ()
				.layer (tower_http::cors::CorsLayer::new ()
					.allow_methods ([ http::Method::GET, http::Method::POST ])
					.allow_origin (tower_http::cors::Any)))
			.with_state (state.clone ());

	let listener =
		tokio::net::TcpListener::bind (& * state.config.core.listen)
			.await.unwrap ();

	axum::serve (listener, app)
		.with_graceful_shutdown (shutdown_signal ())
		.await ?;

	Ok (())

}

struct GlobalState {
	config: Arc <Config>,
	google_auth: GoogleAuth,
	http: reqwest::Client,
}

async fn shutdown_signal () {

	let ctrl_c = async {
		tokio::signal::ctrl_c()
			.await
			.expect ("failed to install Ctrl+C handler");
		eprintln! ("Received INT signal, shutting down...");
	};

	let terminate = async {
		tokio::signal::unix::signal (
				tokio::signal::unix::SignalKind::terminate ())
			.expect ("failed to install signal handler")
			.recv ()
			.await;
		eprintln! ("Received TERM signal, shutting down...");
	};

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

}

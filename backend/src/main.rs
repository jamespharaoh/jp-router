use axum::Json;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing as r;

use chrono::TimeZone;
use chrono::Utc;

use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;

use jp_router_common::*;

#[ tokio::main ]
async fn main () -> anyhow::Result <()> {
    let app =
    	axum::Router::new ()
    		.route ("/dhcp-leases", r::get (dhcp_leases))
			.layer (tower::ServiceBuilder::new ()
				.layer (tower_http::cors::CorsLayer::new ()
					.allow_methods ([ http::Method::GET, http::Method::POST ])
					.allow_origin (tower_http::cors::Any)));
    let listener =
    	tokio::net::TcpListener::bind ("0.0.0.0:3000")
    		.await.unwrap ();
    axum::serve (listener, app).await.unwrap ();
    Ok (())
}

async fn dhcp_leases () -> Result <Json <Vec <DhcpLease>>, ErrorResponse> {
	Ok (Json (get_dhcp_leases ().await ?))
}

async fn get_dhcp_leases () -> anyhow::Result <Vec <DhcpLease>> {
	tokio::task::spawn_blocking (get_dhcp_leases_sync).await ?
}

fn get_dhcp_leases_sync () -> anyhow::Result <Vec <DhcpLease>> {
	let reader = BufReader::new (File::open ("/var/lib/misc/dnsmasq.leases") ?);
	let mut leases = Vec::new ();
	leases.push (DhcpLease {
		expiry_time: None,
		mac_address: "d6:f8:fc:49:c0:5d".parse () ?,
		ip_address: "10.109.132.1".parse () ?,
		hostname: Some ("router".to_owned ()),
		client_id: None,
	});
	for line in reader.lines () {
		let line = line ?;
		let parts: Vec <& str> = line.split (' ').collect ();
		anyhow::ensure! (parts.len () == 5);
		leases.push (DhcpLease {
			expiry_time: Utc.timestamp_opt (parts [0].parse () ?, 0).single (),
			mac_address: parts [1].to_owned (),
			ip_address: parts [2].parse () ?,
			hostname: (parts [3] != "*").then (|| parts [3].to_owned ()),
			client_id: (parts [4] != "*").then (|| parts [4].to_owned ()),
		});
	}
	leases.sort_by_key (|lease| lease.ip_address);
	Ok (leases)
}

#[ derive (Debug, thiserror::Error) ]
enum ErrorResponse {
	#[ error ("{0}") ]
	Anyhow (#[ from ] anyhow::Error),
}

impl IntoResponse for ErrorResponse {
	fn into_response (self) -> Response <Body> {
		match self {
			Self::Anyhow (err) =>
				Response::builder ()
					.body (Body::new (err.to_string ()))
					.unwrap ()
		}
	}
}

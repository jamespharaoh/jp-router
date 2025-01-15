use super::*;

pub fn router () -> Router <Arc <GlobalState>> {
   	axum::Router::new ()
   		.route ("/{domain}", r::put (put))
   		.route ("/{domain}", r::delete (delete))
}

async fn put (
	ex::State (state): ex::State <Arc <GlobalState>>,
	ex::TypedHeader (ex::Authorization (auth)): ex::AuthBasic,
	ex::Path (domain): ex::Path <ArcStr>,
	value: String,
) -> Result <String, ErrorResponse> {
	let domain = arcstr::format! ("_acme-challenge.{domain}");
	let value = ArcStr::from (value);
	auth_verify (& state, & auth, & domain).await ?;
	if google_api::exists (& state, & domain).await ? {
		google_api::delete (& state, & domain).await ?;
	}
	google_api::create (& state, & domain, & value).await ?;
	Ok ("DNS record created\n".to_owned ())
}

async fn delete (
	ex::State (state): ex::State <Arc <GlobalState>>,
	ex::TypedHeader (ex::Authorization (auth)): ex::AuthBasic,
	ex::Path (domain): ex::Path <ArcStr>,
) -> Result <String, ErrorResponse> {
	let domain = arcstr::format! ("_acme-challenge.{domain}");
	auth_verify (& state, & auth, & domain).await ?;
	google_api::delete (& state, & domain).await ?;
	Ok ("DNS record removed\n".to_owned ())
}

async fn auth_verify (
	state: & Arc <GlobalState>,
	auth: & ex::Basic,
	domain: & ArcStr,
) -> Result <(), ErrorResponse> {
	let user =
		state.config.acme.users.iter ()
			.find (|user| user.name == auth.username ())
			.ok_or_else (|| ErrorResponse::unauthorized (anyhow::format_err! ("Auth failed"))) ?;
	if user.secret != auth.password () {
		return Err (ErrorResponse::unauthorized (anyhow::format_err! ("Auth failed")))
	}
	let full_subdomain = domain
		.strip_suffix (& format! (".{}", state.config.acme.domain))
		.ok_or_else (|| anyhow::format_err! ("Invalid domain: {domain}")) ?;
	let main_subdomain = full_subdomain.rsplit ('.').next ().unwrap ().to_string ();
	if ! user.subdomains.iter ()
			.any (|subdomain| & * subdomain == & * main_subdomain) {
		return Err (ErrorResponse::forbidden (anyhow::format_err! ("Access denied")));
	}
	Ok (())
}

mod google_api {

	use super::*;

	pub async fn create (
		state: & Arc <GlobalState>,
		domain: & ArcStr,
		value: & ArcStr,
	) -> Result <(), ErrorResponse> {
		let req_url = format! (
			"https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{zone}/rrsets",
			project = url_encode (& state.config.google_cloud.project_id),
			zone = url_encode (& state.config.acme.cloud_zone));
		let req_body = google_api::ResourceRecordSet {
			name: Some (format! ("{domain}.").into ()),
			type_: Some (arcstr::literal! ("TXT")),
			ttl: Some (60),
			rrdatas: Some (vec! [ value.into () ]),
		};
		let resp = state.http.post (& req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.json (& req_body)
			.send ().await ?;
		if resp.status () == HttpStatus::OK { return Ok (()) }
		let resp_body = resp.text ().await ?;
		eprintln! ("Error calling {req_url}:\n{resp_body}");
		Err (ErrorResponse::internal (anyhow::format_err! ("Error creating DNS records")))
	}

	pub async fn exists (
		state: & Arc <GlobalState>,
		domain: & ArcStr,
	) -> Result <bool, ErrorResponse> {
		let req_url = format! (
			"https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{zone}/rrsets/{domain}/TXT",
			project = url_encode (& state.config.google_cloud.project_id),
			zone = url_encode (& state.config.acme.cloud_zone),
			domain = url_encode (& format! ("{domain}.")));
		let resp = state.http.get (& req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.send ().await ?;
		if resp.status () == HttpStatus::OK { return Ok (true) }
		if resp.status () == HttpStatus::NOT_FOUND { return Ok (false) }
		let resp_body = resp.text ().await ?;
		eprintln! ("Error calling {req_url}:\n{resp_body}");
		Err (ErrorResponse::internal (anyhow::format_err! ("Error creating DNS records")))
	}

	pub async fn delete (
		state: & Arc <GlobalState>,
		domain: & ArcStr,
	) -> Result <(), ErrorResponse> {
		let req_url = format! (
			"https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{zone}/rrsets/{domain}/TXT",
			project = url_encode (& state.config.google_cloud.project_id),
			zone = url_encode (& state.config.acme.cloud_zone),
			domain = url_encode (& format! ("{domain}.")));
		let resp = state.http.delete (& req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.send ().await ?;
		if resp.status () == HttpStatus::OK { return Ok (()) }
		if resp.status () == HttpStatus::NOT_FOUND { return Ok (()) }
		let resp_body = resp.text ().await ?;
		eprintln! ("Error calling {req_url}:\n{resp_body}");
		Err (ErrorResponse::internal (anyhow::format_err! ("Error creating DNS records")))
	}

	#[ derive (Debug, Deserialize, Serialize) ]
	#[ serde (rename_all = "camelCase") ]
	pub struct ResourceRecordSet {
		pub name: Option <ArcStr>,
		pub type_: Option <ArcStr>,
		pub ttl: Option <u64>,
		pub rrdatas: Option <Vec <ArcStr>>,
	}

}

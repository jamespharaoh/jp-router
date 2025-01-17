use super::*;

use google::dns;

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
	let rrset_id = dns::ResourceRecordSetId {
		project: state.config.google_cloud.project_id.clone (),
		zone: state.config.acme.cloud_zone.clone (),
		name: arcstr::format! ("{domain}."),
		type_: arcstr::literal! ("TXT"),
	};
	if let Some (mut rrset) = rrset_id.get (& state).await ? {
		rrset.ttl = 60;
		rrset.rrdatas = vec! [ value.into () ];
		rrset.update (& state).await ?;
		Ok ("DNS record updated\n".to_owned ())
	} else {
		dns::ResourceRecordSet {
			id: rrset_id,
			ttl: 60,
			rrdatas: vec! [ value.into () ],
		}.create (& state).await ?;
		Ok ("DNS record created\n".to_owned ())
	}
}

async fn delete (
	ex::State (state): ex::State <Arc <GlobalState>>,
	ex::TypedHeader (ex::Authorization (auth)): ex::AuthBasic,
	ex::Path (domain): ex::Path <ArcStr>,
) -> Result <String, ErrorResponse> {
	let domain = arcstr::format! ("_acme-challenge.{domain}");
	auth_verify (& state, & auth, & domain).await ?;
	let rrset_id = dns::ResourceRecordSetId {
		project: state.config.google_cloud.project_id.clone (),
		zone: state.config.acme.cloud_zone.clone (),
		name: arcstr::format! ("{domain}."),
		type_: arcstr::literal! ("TXT"),
	};
	if let Some (rrset) = rrset_id.get (& state).await ? {
		rrset.delete (& state).await ?;
	}
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

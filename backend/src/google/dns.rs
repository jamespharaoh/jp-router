use super::*;

#[ derive (Clone, Debug, Default, Deserialize, Serialize) ]
#[ serde (rename_all = "camelCase") ]
pub struct ResourceRecordSetId {
	#[ serde (skip) ]
	pub project: ArcStr,
	#[ serde (skip) ]
	pub zone: ArcStr,
	pub name: ArcStr,
	pub type_: ArcStr,
}

impl ResourceRecordSetId {

	pub fn url (& self) -> ArcStr {
		arcstr::format! (
			"https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{zone}/rrsets/{domain}/{type_}",
			project = url_encode_path (& self.project),
			zone = url_encode_path (& self.zone),
			domain = url_encode_path (& self.name),
			type_ = url_encode_path (& self.type_))
	}

	pub fn parent_url (& self) -> ArcStr {
		arcstr::format! (
			"https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{zone}/rrsets",
			project = url_encode_path (& self.project),
			zone = url_encode_path (& self.zone))
	}

	pub async fn get (
		& self,
		state: & GlobalState,
	) -> anyhow::Result <Option <ResourceRecordSet>> {
		let req_url = self.url ();
		let resp = state.http.get (& * req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.send ().await ?;
		match resp.status () {
			HttpStatus::OK => {
				let mut resp_body: ResourceRecordSet = resp.json ().await ?;
				resp_body.id = self.clone ();
				Ok (Some (resp_body))
			},
			HttpStatus::NOT_FOUND => Ok (None),
			status => {
				let resp_body = resp.text ().await ?;
				Err (anyhow::format_err! ("Error getting {req_url}: {status}: {resp_body}"))
			},
		}
	}

}

#[ derive (Debug, Default, Deserialize, Serialize) ]
#[ serde (rename_all = "camelCase") ]
pub struct ResourceRecordSet {
	#[ serde (flatten) ]
	pub id: ResourceRecordSetId,
	pub ttl: u64,
	pub rrdatas: Vec <ArcStr>,
}

impl ResourceRecordSet {

	pub async fn create (
		& self,
		state: & GlobalState,
	) -> anyhow::Result <()> {
		let req_url = self.id.parent_url ();
		let resp = state.http.post (& * req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.json (self)
			.send ().await ?;
		match resp.status () {
			HttpStatus::OK => Ok (()),
			status => {
				let resp_body = resp.text ().await ?;
				Err (anyhow::format_err! ("Error posting {req_url}: {status}: {resp_body}"))
			},
		}
	}

	pub async fn update (
		& self,
		state: & GlobalState,
	) -> anyhow::Result <()> {
		let req_url = self.id.url ();
		let resp = state.http.patch (& * req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.json (self)
			.send ().await ?;
		match resp.status () {
			HttpStatus::OK => Ok (()),
			HttpStatus::NOT_FOUND => Ok (()),
			status => {
				let resp_body = resp.text ().await ?;
				Err (anyhow::format_err! ("Error patching {req_url}: {status}: {resp_body}"))
			},
		}
	}

	pub async fn delete (
		& self,
		state: & GlobalState,
	) -> anyhow::Result <()> {
		let req_url = self.id.url ();
		let resp = state.http.delete (& * req_url)
			.header ("authorization", state.google_auth.token ().await ?.to_string ())
			.send ().await ?;
		match resp.status () {
			HttpStatus::OK => Ok (()),
			HttpStatus::NOT_FOUND => Ok (()),
			status => {
				let resp_body = resp.text ().await ?;
				Err (anyhow::format_err! ("Error deleting {req_url}: {status}: {resp_body}"))
			},
		}
	}

}

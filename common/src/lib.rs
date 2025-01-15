use arcstr::ArcStr;

use chrono::DateTime;
use chrono::Utc;

use serde::Deserialize;
use serde::Serialize;

use std::net::IpAddr;

#[ derive (Deserialize, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub struct DhcpLease {
	pub expiry_time: Option <DateTime <Utc>>,
	pub mac_address: ArcStr,
	pub ip_address: IpAddr,
	pub hostname: Option <ArcStr>,
	pub client_id: Option <ArcStr>,
}

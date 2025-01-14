use chrono::DateTime;
use chrono::Utc;

use mac_address::MacAddress;

use serde::Deserialize;
use serde::Serialize;

use std::net::IpAddr;

#[ derive (Deserialize, Serialize) ]
pub struct DhcpLease {
	pub expiry_time: Option <DateTime <Utc>>,
	pub mac_address: MacAddress,
	pub ip_address: IpAddr,
	pub hostname: Option <String>,
	pub client_id: Option <String>,
}

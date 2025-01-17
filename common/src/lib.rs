use arcstr::ArcStr;

use chrono::DateTime;
use chrono::Utc;

use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;

#[ derive (Clone, Debug, Deserialize, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub struct DhcpLease {
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub expiry_time: Option <DateTime <Utc>>,
	pub mac_address: ArcStr,
	pub ip_address: IpAddr,
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub hostname: Option <ArcStr>,
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub client_id: Option <ArcStr>,
}

#[ derive (Clone, Debug, Deserialize, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub struct NetworkInterface {
	pub index: u32,
	pub name: ArcStr,
	pub oper_state: NetworkInterfaceOperState,
	pub mac_address: ArcStr,
	pub ip_addresses: Vec <NetworkAddress>,
	pub mtu: u32,
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub speed: Option <u64>,
	pub rx_bytes: u64,
	pub rx_dropped: u64,
	pub rx_errors: u64,
	pub rx_packets: u64,
	pub tx_bytes: u64,
	pub tx_dropped: u64,
	pub tx_errors: u64,
	pub tx_packets: u64,
}

#[ derive (Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub enum NetworkInterfaceOperState {
	Unknown,
	NotPresent,
	Down,
	LowerLayerDown,
	Testing,
	Dormant,
	Up,
	Other (u8),
	None,
}

impl fmt::Display for NetworkInterfaceOperState {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Unknown => fmtr.write_str ("unknown") ?,
			Self::NotPresent => fmtr.write_str ("not-present") ?,
			Self::Down => fmtr.write_str ("down") ?,
			Self::LowerLayerDown => fmtr.write_str ("lower-layer-down") ?,
			Self::Testing => fmtr.write_str ("testing") ?,
			Self::Dormant => fmtr.write_str ("dormant") ?,
			Self::Up => fmtr.write_str ("up") ?,
			Self::Other (val) => write! (fmtr, "other ({val}") ?,
			Self::None => fmtr.write_str ("none") ?
		}
		Ok (())
	}
}

#[ derive (Clone, Debug, Deserialize, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub struct NetworkAddress {
	pub scope: NetworkAddressScope,
	pub address: IpAddr,
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub local: Option <IpAddr>,
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub broadcast: Option <Ipv4Addr>,
	pub prefix_len: u8,
}

#[ derive (Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize) ]
#[ serde (rename_all = "kebab-case") ]
pub enum NetworkAddressScope {
	Universe,
	Site,
	Link,
	Host,
	Nowhere,
	Other (u8),
	None,
}

use super::*;

use netlink_packet_core::NLM_F_DUMP;
use netlink_packet_core::NLM_F_REQUEST;
use netlink_packet_core::NetlinkMessage;
use netlink_packet_core::NetlinkPayload;
use netlink_packet_route::AddressFamily;
use netlink_packet_route::RouteNetlinkMessage;
use netlink_packet_route::address::AddressAttribute;
use netlink_packet_route::address::AddressMessage;
use netlink_packet_route::address::AddressScope;
use netlink_packet_route::link::LinkAttribute;
use netlink_packet_route::link::LinkMessage;
use netlink_packet_route::link::State as LinkState;
use netlink_proto::ConnectionHandle;
use netlink_proto::sys::protocols::NETLINK_ROUTE;

pub fn router () -> Router <Arc <GlobalState>> {
	axum::Router::new ()
		.route ("/", r::get (get))
}

async fn get () -> Result <Json <Vec <NetworkInterface>>, ErrorResponse> {
	Ok (Json (fetch ().await ?))
}

async fn fetch () -> anyhow::Result <Vec <NetworkInterface>> {
	let mut ifaces = Vec::new ();
	let (nl_conn, nl_handle, _nl_messages) =
		netlink_proto::new_connection (
			NETLINK_ROUTE) ?;
	tokio::spawn (nl_conn);
	let addrs: HashMap <u32, Vec <NetworkAddress>> =
		get_addresses (& nl_handle).await ?.into_iter ()
			.filter_map (|addr| {
				let mut address = None;
				let mut local = None;
				let mut broadcast = None;
				for attr in addr.attributes {
					match attr {
						AddressAttribute::Address (val) => address = Some (val),
						AddressAttribute::Local (val) => local = Some (val),
						AddressAttribute::Broadcast (val) => broadcast = Some (val),
						_ => (),
					}
				}
				let Some (address) = address else { return None };
				Some ((
					addr.header.index,
					NetworkAddress {
						scope: match addr.header.scope {
							AddressScope::Universe => NetworkAddressScope::Universe,
							AddressScope::Site => NetworkAddressScope::Site,
							AddressScope::Link => NetworkAddressScope::Link,
							AddressScope::Host => NetworkAddressScope::Host,
							AddressScope::Nowhere => NetworkAddressScope::Nowhere,
							AddressScope::Other (val) => NetworkAddressScope::Other (val),
							_ => NetworkAddressScope::None,
						},
						prefix_len: addr.header.prefix_len,
						address,
						local,
						broadcast,
					},
				))
			})
			.into_group_map ();
	for link in get_links (& nl_handle).await ? {
		let mut name = None;
		let mut oper_state = None;
		let mut mac_address = None;
		let mut mtu = None;
		let mut stats = None;
		for attr in link.attributes {
			match attr {
				LinkAttribute::IfName (val) => name = Some (ArcStr::from (val)),
				LinkAttribute::OperState (val) => oper_state = Some (match val {
					LinkState::Unknown => NetworkInterfaceOperState::Unknown,
					LinkState::NotPresent => NetworkInterfaceOperState::NotPresent,
					LinkState::Down => NetworkInterfaceOperState::Down,
					LinkState::LowerLayerDown => NetworkInterfaceOperState::LowerLayerDown,
					LinkState::Testing => NetworkInterfaceOperState::Testing,
					LinkState::Dormant => NetworkInterfaceOperState::Dormant,
					LinkState::Up => NetworkInterfaceOperState::Up,
					LinkState::Other (val) => NetworkInterfaceOperState::Other (val),
					_ => NetworkInterfaceOperState::None,
				}),
				LinkAttribute::Address (val) => mac_address = Some (ArcStr::from (
					val.iter ().map (|byte| format! ("{byte:02x}")).join (":"))),
				LinkAttribute::Mtu (val) => mtu = Some (val),
				LinkAttribute::Stats64 (val) => stats = Some (val),
				_ => (),
			}
		}
		let Some (name) = name else { continue };
		let Some (oper_state) = oper_state else { continue };
		let Some (mac_address) = mac_address else { continue };
		let Some (mtu) = mtu else { continue };
		let Some (stats) = stats else { continue };
		let link_sys_path = Path::new ("/sys/class/net").join (& * name);
		ifaces.push (NetworkInterface {
			index: link.header.index,
			name,
			oper_state,
			mac_address,
			ip_addresses: addrs.get (& link.header.index).cloned ().unwrap_or_default (),
			mtu,
			speed: tokio::fs::read_to_string (link_sys_path.join ("speed")).await
				.map_err (anyhow::Error::new)
				.and_then (|val| val.trim ().parse ().map_err (anyhow::Error::new))
				.ok (),
			rx_bytes: stats.rx_bytes,
			rx_dropped: stats.rx_dropped,
			rx_errors: stats.rx_errors,
			rx_packets: stats.rx_packets,
			tx_bytes: stats.tx_bytes,
			tx_dropped: stats.tx_dropped,
			tx_errors: stats.tx_errors,
			tx_packets: stats.tx_packets,
		});
	}
	Ok (ifaces)
}

async fn get_links (
	nl_handle: & ConnectionHandle <RouteNetlinkMessage>,
) -> anyhow::Result <Vec <LinkMessage>> {
	let mut nl_req_msg = LinkMessage::default ();
	nl_req_msg.header.interface_family = AddressFamily::Unspec;
	let mut nl_req =
		NetlinkMessage::from (
			RouteNetlinkMessage::GetLink (
				nl_req_msg));
	nl_req.header.flags = netlink_packet_core::NLM_F_REQUEST | netlink_packet_core::NLM_F_DUMP;
	nl_req.header.sequence_number = 1;
	nl_req.finalize ();
	let kernel_unicast = netlink_proto::sys::SocketAddr::new (0, 0);
	let nl_stream = nl_handle.request (nl_req, kernel_unicast) ?;
	Ok (
		nl_stream
			.filter_map (|nl_resp| async { match nl_resp.payload {
				NetlinkPayload::InnerMessage (RouteNetlinkMessage::NewLink (new_link)) => Some (new_link),
				_ => None,
			}})
			.collect ()
			.await
	)
}

async fn get_addresses (
	nl_handle: & ConnectionHandle <RouteNetlinkMessage>,
) -> anyhow::Result <Vec <AddressMessage>> {
	let mut nl_req_msg = AddressMessage::default ();
	nl_req_msg.header.family = AddressFamily::Unspec;
	let mut nl_req =
		NetlinkMessage::from (
			RouteNetlinkMessage::GetAddress (
				nl_req_msg));
	nl_req.header.flags = NLM_F_DUMP | NLM_F_REQUEST;
	nl_req.header.sequence_number = 1;
	nl_req.finalize ();
	let kernel_unicast = netlink_proto::sys::SocketAddr::new (0, 0);
	let nl_stream = nl_handle.request (nl_req, kernel_unicast) ?;
	Ok (
		nl_stream
			.filter_map (|nl_resp| async { match nl_resp.payload {
				NetlinkPayload::InnerMessage (RouteNetlinkMessage::NewAddress (address)) =>
					Some (address),
				_ => None,
			}})
			.collect ()
			.await
	)
}

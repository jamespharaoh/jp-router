use super::*;

use google::dns;

pub async fn run (state: & GlobalState) {
	if ! state.config.dynamic_dns.enabled { return }
	log::info! ("Started");
	let mut interval = tokio::time::interval (
		StdDuration::from_secs (state.config.dynamic_dns.interval_secs));
	interval.set_missed_tick_behavior (tokio::time::MissedTickBehavior::Skip);
	loop {
		interval.tick ().await;
		if let Err (err) = run_once (state).await {
			log::error! ("Error: {err:?}");
		}
	}
}

async fn run_once (state: & GlobalState) -> anyhow::Result <()> {
	let networks = networks::fetch ().await ?;
	let Some (wan_iface) = networks.iter ()
				.find (|net| net.name == state.config.core.wan_iface)
			else {
		log::warn! ("WAN interface not found");
		return Ok (())
	};
	let Some (wan_addr) = wan_iface.ip_addresses.iter ()
				.filter (|addr| addr.scope == NetworkAddressScope::Universe)
				.filter_map (|addr| match (addr.address, addr.local) {
					(_, Some (IpAddr::V4 (addr))) => Some (addr),
					(IpAddr::V4 (addr), None) => Some (addr),
					_ => None,
				})
				.next ()
			else {
		log::warn! ("WAN interface has no public address");
		return Ok (())
	};
	let rrset_id = dns::ResourceRecordSetId {
		project: state.config.google_cloud.project_id.clone (),
		zone: state.config.dynamic_dns.cloud_zone.clone (),
		name: arcstr::format! ("{}.", state.config.dynamic_dns.domain),
		type_: arcstr::literal! ("A"),
	};
	if let Some (mut rrset) = rrset_id.get (& state).await ? {
		if rrset.ttl != 60 && rrset.rrdatas != & [ arcstr::format! ("{wan_addr}") ] {
			rrset.ttl = 60;
			rrset.rrdatas = vec! [ arcstr::format! ("{wan_addr}") ];
			rrset.update (& state).await ?;
			log::info! ("DNS record updated to {wan_addr}");
		}
	} else {
		let rrset = dns::ResourceRecordSet {
			id: rrset_id,
			ttl: 60,
			rrdatas: vec! [ arcstr::format! ("{wan_addr}") ],
		};
		rrset.create (& state).await ?;
		log::info! ("DNS record created for {wan_addr}");
	}
	Ok (())
}

use super::*;

pub fn router () -> Router <Arc <GlobalState>> {
   	axum::Router::new ()
   		.route ("/", r::get (get))
}

async fn get () -> Result <Json <Vec <DhcpLease>>, ErrorResponse> {
	Ok (Json (fetch ().await ?))
}

async fn fetch () -> anyhow::Result <Vec <DhcpLease>> {
	tokio::task::spawn_blocking (fetch_sync).await ?
}

fn fetch_sync () -> anyhow::Result <Vec <DhcpLease>> {
	let reader = BufReader::new (File::open ("/var/lib/misc/dnsmasq.leases") ?);
	let mut leases = Vec::new ();
	leases.push (DhcpLease {
		expiry_time: None,
		mac_address: "d6:f8:fc:49:c0:5d".parse () ?,
		ip_address: "10.109.132.1".parse () ?,
		hostname: Some (arcstr::literal! ("router")),
		client_id: None,
	});
	for line in reader.lines () {
		let line = line ?;
		let parts: Vec <& str> = line.split (' ').collect ();
		anyhow::ensure! (parts.len () == 5);
		leases.push (DhcpLease {
			expiry_time: Utc.timestamp_opt (parts [0].parse () ?, 0).single (),
			mac_address: parts [1].into (),
			ip_address: parts [2].parse () ?,
			hostname: (parts [3] != "*").then (|| parts [3].into ()),
			client_id: (parts [4] != "*").then (|| parts [4].into ()),
		});
	}
	leases.sort_by_key (|lease| lease.ip_address);
	Ok (leases)
}

use super::*;

#[ function_component (DhcpLeases) ]
pub fn dhcp_leases () -> Html {
	let leases = use_state (|| Vec::new ());
	let update = {
		let leases = leases.clone ();
		move || {
			let leases = leases.clone ();
			wasm_bindgen_futures::spawn_local (async move {
				match fetch_leases ().await {
					Ok (fetched_leases) => {
						log::info! ("Fetched {} DHCP leases", fetched_leases.len ());
						leases.set (fetched_leases);
					},
					Err (err) => {
						log::error! ("Error fetching DHCP leases: {err}");
					},
				};
			});
		}
	};
	use_interval (update.clone (), 5000);
	use_effect_with ((), move |_| update ());
	html! {
		<Template>
			<section>
				<h2>{ "DHCP leases" }</h2>
				<table>
					<thead>
						<tr>
							<th>{ "Expiry time" }</th>
							<th>{ "MAC address" }</th>
							<th>{ "IP address" }</th>
							<th>{ "Hostname" }</th>
							<th>{ "Client ID" }</th>
						</tr>
					</thead>
					<tbody> {
						leases.iter ()
							.map (|lease| html! {
								<tr key={ lease.ip_address.to_string () }>
									<td>{
										lease.expiry_time
											.map (|val| DateTime::<Local>::from (val)
												.format ("%Y-%m-%d %H:%M:%S")
												.to_string ())
											.unwrap_or_default ()
									}</td>
									<td>{ lease.mac_address.as_str () }</td>
									<td>{ lease.ip_address.to_string () }</td>
									<td>{ lease.hostname.as_ref ().map (ArcStr::as_str) }</td>
									<td>{ lease.client_id.as_ref ().map (ArcStr::as_str) }</td>
								</tr>
							})
							.collect::<Html> ()
					} </tbody>
				</table>
			</section>
		</Template>
	}
}

async fn fetch_leases () -> anyhow::Result <Vec <DhcpLease>> {
	Ok (
		gloo_net::http::Request::get ("http://router.arago136.es/api/dhcp-leases")
			.send ().await ?
			.json ().await ?
	)
}

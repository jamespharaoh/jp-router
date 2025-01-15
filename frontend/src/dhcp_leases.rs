use super::*;

#[ function_component (DhcpLeases) ]
pub fn dhcp_leases () -> Html {
	fn update (leases: UseStateHandle <Vec <DhcpLease>>) {
		wasm_bindgen_futures::spawn_local (async move {
			let fetched_leases = fetch_leases ().await;
			log::info! ("fetched {} leases", fetched_leases.len ());
			leases.set (fetched_leases);
		});
	}
	let leases = use_state (|| Vec::new ());
	use_interval ({
		let leases = leases.clone ();
		move || update (leases.clone ())
	}, 5000);
	use_effect_with ((), {
		let leases = leases.clone ();
		move |_| { update (leases) }
	});
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
									<td>{ & lease.mac_address }</td>
									<td>{ lease.ip_address.to_string () }</td>
									<td>{ lease.hostname.as_ref () }</td>
									<td>{ lease.client_id.as_ref () }</td>
								</tr>
							})
							.collect::<Html> ()
					} </tbody>
				</table>
			</section>
		</Template>
	}
}

async fn fetch_leases () -> Vec <DhcpLease> {
	// TODO handle errors
	gloo_net::http::Request::get (
			"http://router.arago136.es:3000/dhcp-leases")
		.send ()
		.await
		.unwrap ()
		.json ()
		.await
		.unwrap ()
}

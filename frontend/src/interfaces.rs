use super::*;

#[ function_component (Interfaces) ]
pub fn interfaces () -> Html {
	let ifaces = use_state (|| Vec::new ());
	let update = {
		let ifaces = ifaces.clone ();
		move || {
			let ifaces = ifaces.clone ();
			wasm_bindgen_futures::spawn_local (async move {
				match fetch_ifaces ().await {
					Ok (fetched_ifaces) => {
						log::info! ("Fetched {} interfaces", fetched_ifaces.len ());
						ifaces.set (fetched_ifaces);
					},
					Err (err) => {
						log::error! ("Error fetching network interfaces: {err}");
					},
				};
			});
		}
	};
	use_interval (update.clone (), 400);
	use_effect_with ((), move |_| update ());
	html! {
		<>
			<Header/>
			<Nav/>
			<section>
				<header>
					<h1>{ "Network interfaces" }</h1>
				</header>
				<div class="help">
					<p>{ "View configuration information, addresses, and real-time statistics \
						for all network interfaces" }</p>
				</div>
				<table>
					<thead>
						<tr>
							<th>{ "Name" }</th>
							<th>{ "State" }</th>
							<th>{ "MAC address" }</th>
							<th>{ "IP address" }</th>
							<th>{ "Speed" }</th>
							<th>{ "MTU" }</th>
							<th colspan="4">{ "RX" }</th>
							<th colspan="4">{ "TX" }</th>
						</tr>
					</thead>
					<tbody> {
						ifaces.iter ()
							.map (|iface| html! {
								<tr key={ & * iface.name }>
									<td>{ & * iface.name }</td>
									<td>{ iface.oper_state.to_string () }</td>
									<td>{ & * iface.mac_address }</td>
									<td>{ iface.ip_addresses.iter ()
										.map (|addr| html! {
											<>
												{ & * addr.address.to_string () }
												{ "/" }
												{ addr.prefix_len }
												<br/>
											</>
										})
										.collect::<Html> ()
									}</td>
									<td>{ iface.speed }</td>
									<td>{ iface.mtu }</td>
									<td>{ iface.rx_packets }</td>
									<td>{ iface.rx_bytes }</td>
									<td>{ iface.rx_dropped }</td>
									<td>{ iface.rx_errors }</td>
									<td>{ iface.tx_packets }</td>
									<td>{ iface.tx_bytes }</td>
									<td>{ iface.tx_dropped }</td>
									<td>{ iface.tx_errors }</td>
								</tr>
							})
							.collect::<Html> ()
					} </tbody>
				</table>
			</section>
		</>
	}
}

async fn fetch_ifaces () -> anyhow::Result <Vec <NetworkInterface>> {
	Ok (
		gloo_net::http::Request::get ("https://router.arago136.es/api/networks")
			.send ().await ?
			.json ().await ?
	)
}

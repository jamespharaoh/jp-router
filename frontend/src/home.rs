use super::*;

#[ function_component (Home) ]
pub fn home () -> Html {
	html! {
		<>
			<Header/>
			<Nav/>
			<section>
				<header>
					<h1>{ "Links" }</h1>
				</header>
				<div class="help">
					<p>{ "Links to other services running on this device" }</p>
				</div>
				<li><a href="/bandwidthd/">{ "BandwidthD" }</a></li>
				<li><a href="/darkstat/">{ "DarkStat" }</a></li>
				<li><a href="/netdata/">{ "Netdata" }</a></li>
				<li><a href="https://pihole.arago136.es/">{ "Pi-hole" }</a></li>
			</section>
		</>
	}
}

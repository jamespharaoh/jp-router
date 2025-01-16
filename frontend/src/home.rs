use super::*;

#[ function_component (Home) ]
pub fn home () -> Html {
	html! {
		<Template>
			<section>
				<h2>{ "Home" }</h2>
				<h3>{ "Links" }</h3>
				<li><a href="/bandwidthd/">{ "BandwidthD" }</a></li>
				<li><a href="/darkstat/">{ "DarkStat" }</a></li>
				<li><a href="/netdata/">{ "Netdata" }</a></li>
				<li><a href="https://pihole.arago136.es/">{ "Pi-hole" }</a></li>
			</section>
		</Template>
	}
}

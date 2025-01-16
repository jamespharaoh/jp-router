use super::*;

#[ function_component (Header) ]
pub fn header () -> Html {
	html! {
		<header>
			<h1>{ "router.arago136.es" }</h1>
			<div class="logo-container">
				<div class="logo"/>
			</div>
		</header>
	}
}

#[ function_component (Nav) ]
pub fn nav () -> Html {
	html! {
		<nav>
			<Link <Route> to={ Route::Home }>{ "Home" }</Link <Route>>
			{ " | " }
			<Link <Route> to={ Route::DhcpLeases }>{ "DHCP Leases" }</Link <Route>>
			{ " | " }
			<Link <Route> to={ Route::Interfaces }>{ "Network interfaces" }</Link <Route>>
		</nav>
	}
}

use super::*;

pub fn switch (routes: Route) -> Html {
	match routes {
		Route::Home => html! { <Home/> },
		Route::DhcpLeases => html! { <DhcpLeases/> },
		Route::NotFound => html! { <NotFound/> },
	}
}

#[ derive (Clone, Debug, Eq, PartialEq, Routable) ]
pub enum Route {
	#[ at ("/") ]
	Home,
	#[ at ("/dhcp-leases") ]
	DhcpLeases,
	#[ not_found ]
	#[ at ("/404") ]
	NotFound,
}

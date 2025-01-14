use super::*;

#[ derive (Properties, PartialEq) ]
pub struct TemplateProps {
	pub children: Children,
}

#[ function_component (Template) ]
pub fn template (props: & TemplateProps) -> Html {
	html! {
		<>
			<header>
				<h1>{ "JP Router" }</h1>
			</header>
			<nav>
				<Link <Route> to={ Route::Home }>{ "Home" }</Link <Route>>
				{ " | " }
				<Link <Route> to={ Route::DhcpLeases }>{ "DHCP Leases" }</Link <Route>>
			</nav>
			{ props.children.clone () }
		</>
	}
}

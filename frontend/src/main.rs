use arcstr::ArcStr;

use chrono::DateTime;
use chrono::Local;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use jp_router_common::*;

mod dhcp_leases;
mod home;
mod template;
mod route;

use dhcp_leases::*;
use home::*;
use template::*;
use route::*;

fn main () {
	wasm_logger::init (wasm_logger::Config::default ());
	yew::Renderer::<App>::new ().render ();
}

#[ function_component ]
fn App () -> Html {
	html! {
			<BrowserRouter>
				<Switch <Route> render={ switch }/>
			</BrowserRouter>
	}
}

#[ function_component ]
fn NotFound () -> Html {
	html! {
		<section>
			<h2>{ "Not Found" }</h2>
			<p>{ "Page not found" }</p>
		</section>
	}
}

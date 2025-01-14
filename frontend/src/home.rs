use super::*;

#[ function_component (Home) ]
pub fn home () -> Html {
	html! {
		<Template>
			<section>
				<h2>{ "Home" }</h2>
				<p>{ "TODO" }</p>
			</section>
		</Template>
	}
}

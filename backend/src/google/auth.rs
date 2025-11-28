use super::*;

pub struct GoogleAuth {
	token_source: Arc <dyn token_source::TokenSource>,
}

impl GoogleAuth {

	pub async fn build (config: & Arc <Config>) -> anyhow::Result <GoogleAuth> {
		let credentials =
			gcloud_auth::credentials::CredentialsFile::new_from_str (
					& config.google_cloud.credentials)
				.await ?;
		let config =
			gcloud_auth::project::Config::default ()
				.with_audience ("https://dns.googleapis.com/")
				.with_scopes (& [
					"https://www.googleapis.com/auth/cloud-platform",
					"https://www.googleapis.com/auth/ndev.clouddns.readwrite",
				]);
		let token_provider =
			gcloud_auth::token::DefaultTokenSourceProvider::new_with_credentials (
					config,
					Box::new (credentials))
				.await ?;
		let token_source =
			token_provider.token_source ();
		Ok (GoogleAuth { token_source })
	}

	pub async fn token (& self) -> anyhow::Result <ArcStr> {
		self.token_source.token ().await
			.map (ArcStr::from)
			.map_err (|err| anyhow::format_err! ("Error getting Google access token: {err}"))
	}

}

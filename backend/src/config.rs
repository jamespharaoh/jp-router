use super::*;

pub async fn load_config () -> anyhow::Result <Arc <Config>> {
	tokio::task::spawn_blocking (|| load_config_sync ()).await ?
}

fn load_config_sync () -> anyhow::Result <Arc <Config>> {
	let file = File::open ("config.yaml") ?;
	let config: Arc <Config> = serde_yaml::from_reader (file) ?;
	Ok (config)
}

#[ derive (Deserialize) ]
#[ serde (deny_unknown_fields, rename_all = "kebab-case") ]
pub struct Config {
	pub acme: Arc <AcmeConfig>,
	pub core: Arc <CoreConfig>,
	pub google_cloud: Arc <GoogleCloudConfig>,
}

#[ derive (Deserialize) ]
#[ serde (deny_unknown_fields, rename_all = "kebab-case") ]
pub struct AcmeConfig {
	pub cloud_zone: ArcStr,
	pub domain: ArcStr,
	pub users: Vec <AcmeUserConfig>,
}

#[ derive (Deserialize) ]
#[ serde (deny_unknown_fields, rename_all = "kebab-case") ]
pub struct AcmeUserConfig {
	pub name: ArcStr,
	pub secret: ArcStr,
	pub subdomains: Vec <ArcStr>,
}

#[ derive (Deserialize) ]
#[ serde (deny_unknown_fields, rename_all = "kebab-case") ]
pub struct CoreConfig {
	pub lan_bridge_iface: ArcStr,
	pub lan_iface: ArcStr,
	pub listen: ArcStr,
	pub wan_iface: ArcStr,
}

#[ derive (Deserialize) ]
#[ serde (deny_unknown_fields, rename_all = "kebab-case") ]
pub struct GoogleCloudConfig {
	pub credentials: ArcStr,
	pub project_id: ArcStr,
}

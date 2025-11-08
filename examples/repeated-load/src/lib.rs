use rse_std::prelude::*;
use std::convert::Infallible;

struct RepeatedLoad;
impl Plugin for RepeatedLoad {
	type LoadError = Infallible;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
		con_msg!("Plugin has been loaded!");
		Ok(Self)
	}

	fn description(&mut self) -> &CStr {
		plugin_description!()
	}

	fn repeated_load(&mut self, factories: PluginFactories) {
		let _ = factories;
		con_warn!("This plugin should not be loaded again! Please unload it first.");
	}
}
export_plugin!(RepeatedLoad);

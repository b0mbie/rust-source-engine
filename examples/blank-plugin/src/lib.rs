use rse_plugin::prelude::*;

struct Blank;
impl LoadablePlugin for Blank {
	fn load(factories: PluginFactories) -> Option<Self> {
		let _ = factories;
		Some(Self)
	}
}
impl Plugin for Blank {
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_loadable_plugin!(Blank);

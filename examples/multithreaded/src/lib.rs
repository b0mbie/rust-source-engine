use rse_std::prelude::*;
use rse_std::thread::{
	running,
	spawn, sleep,
};
use std::time::Duration;

struct MultiThreaded;
impl Plugin for MultiThreaded {
	type LoadError = &'static str;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;

		spawn(move || {
			while running() {
				dev_msg!("Tick");
				sleep(Duration::from_secs(1));
			}
		}).ok_or("couldn't spawn a new thread")?;

		Ok(Self)
	}

	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_plugin!(MultiThreaded);

use ::rse_server_plugin::prelude::*;

struct Test {
	engine_server: VEngineServer,
	event_manager: EventManager,
}

impl Drop for Test {
	fn drop(&mut self) {
		self.engine_server.server_command(c"echo i drank dinner\n");
	}
}

impl LoadablePlugin for Test {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
		let mut engine_server = factories.create_interface::<VEngineServer>().ok()?;
		engine_server.server_command(c"echo i drank water\n");
		let event_manager = factories.create_interface::<EventManager>().ok()?;
		Some(Self {
			engine_server,
			event_manager,
		})
	}
}

impl Plugin for Test {
	fn description(&mut self) -> &CStr {
		// "test-server-plugin v0.1.0"
		unsafe { CStr::from_bytes_with_nul_unchecked(
			concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"), "\0").as_bytes()
		) }
	}
}

export_loadable_plugin!(Test);

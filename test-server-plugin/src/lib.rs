use ::rse_server_plugin::prelude::*;
use ::rse_tier0::prelude::*;

struct Test {
	engine_server: VEngineServer,
	event_manager: GameEventManager2,
}

impl LoadablePlugin for Test {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
		let mut engine_server = factories.create_interface::<VEngineServer>().ok()?;
		warn!(con(), "{:?} moved wrongly!", "niko oneshor");
		engine_server.server_command(c"alias test_reload \"plugin_unload 0;plugin_load addons/libtest_server_plugin\"\n");
		let event_manager = factories.create_interface::<GameEventManager2>().ok()?;
		Some(Self {
			engine_server,
			event_manager,
		})
	}
}

impl Plugin for Test {
	fn description(&mut self) -> &CStr {
		// "test-server-plugin v0.1.0"
		unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(
			::core::concat!(::core::env!("CARGO_PKG_NAME"), " v", ::core::env!("CARGO_PKG_VERSION"), "\0").as_bytes()
		) }
	}
}

export_loadable_plugin!(Test);

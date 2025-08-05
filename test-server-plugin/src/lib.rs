use ::rse_server_plugin::prelude::*;
use ::rse_tier0_log::*;

struct Test {
	engine_server: VEngineServer,
}

impl Drop for Test {
	fn drop(&mut self) {
		log::debug!("Test plugin unloading");
	}
}

impl LoadablePlugin for Test {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
		install_tier0_logger().ok()?;
		log::info!("This is an informational message logged with {:?}", "tier0");
		log::warn!("This is a warning printed with {:?}", "tier0");
		log::warn!("This is what we call an \"ERR-OR\"... printed with tier0");
		log::debug!("This is a debug message only visible with developer mode on");
		log::trace!("This is a trace message, same thing as the above");

		let mut engine_server = factories.create_interface::<VEngineServer>().ok()?;
		engine_server.server_command(c"alias test_reload \"plugin_unload 0;plugin_load addons/libtest_server_plugin\"\n");
		Some(Self {
			engine_server,
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

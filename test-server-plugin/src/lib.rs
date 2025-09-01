use ::rse_server_plugin::prelude::*;
use ::rse_tier0_print::{
	tier0::prelude::*,
	prelude::*,
};

macro_rules! println {
	($t:expr) => {{
		::rse_tier0_print::Printer::print(
			&::rse_tier0_print::tier0::linked::con(),
			::rse_tier0_print::ComposeThen::then(
				$t,
				::rse_tier0_print::IntoPlain::plain(::printf::ByteChar(b'\n')),
			)
		)
	}};
}

struct Test;

impl Drop for Test {
	fn drop(&mut self) {
		println!(
			"Test plugin".rgb::<255, 0, 191>()
				.then(" is ".plain())
				.then("unloading".rgb::<255, 0, 0>())
		);
	}
}

impl LoadablePlugin for Test {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
		con_msg!("This is an informational message logged with {:?}", "tier0");
		con_warn!("This is what we call an \"ERR-OR\"... or, warning, printed with tier0");
		dev_msg!("This is a debug message only visible with developer mode on");
		dev_warn!("This is a developer-facing warning message, same thing as the above");
		con_color_msg!(
			(&Color::rgb(255, 0, 191), "1111 I Am "),
			(&Color::rgb(0, 255, 0), "GRN"),
		);

		let mut engine_server = factories.create_interface::<VEngineServer>().ok()?;
		engine_server.server_command(c"alias test_reload \"plugin_unload 0;plugin_load addons/libtest_server_plugin\"\n");

		let game = factories.create_interface::<ServerGameDll>().ok()?;
		for server_class in game.server_classes() {
			println!(server_class.network_name().plain());
		}

		Some(Self)
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

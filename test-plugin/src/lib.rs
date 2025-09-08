use ::anyhow::{
	Result, Error,
};
use ::rse_plugin::prelude::*;
use ::rse_tier0::prelude::*;

mod std_handlers;

fn handle_anyhow_error(error: Error) {
	let con = con();
	let mut chain = error.chain().peekable();
	while let Some(error) = chain.next() {
		warn!(con, "{error}");
		if chain.peek().is_some() {
			warn!(con, ": ");
		} else {
			warnln!(con);
		}
	}
}

struct Test {
	dll: ServerGameDll,
}

impl Test {
	fn load_impl(factories: InterfaceFactories<'_>) -> Result<Self> {
		con_msg!("This is an informational message logged with {:?}", "tier0");
		con_warn!("This is what we call an \"ERR-OR\"... or, warning, printed with tier0");
		dev_msg!("This is a debug message only visible with developer mode on");
		dev_warn!("This is a developer-facing warning message, same thing as the above");
		con_color_msg!(
			(&Color::rgb(255, 0, 191), "1111 I Am "),
			(&Color::rgb(0, 255, 0), "GRN"),
		);
		
		let mut engine_server = factories.create_interface::<VEngineServer>()?;
		engine_server.server_command(c"alias test_reload \"plugin_unload 0;plugin_load addons/test\"\n");

		let dll = factories.create_interface::<ServerGameDll>()?;
		Ok(Self {
			dll,
		})
	}
}

impl Drop for Test {
	fn drop(&mut self) {
		dev_msg!("Test plugin is unloading");
	}
}

impl LoadablePlugin for Test {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
		match Self::load_impl(factories) {
			Ok(inst) => Some(inst),
			Err(error) => {
				handle_anyhow_error(error);
				None
			}
		}
	}
}

impl Plugin for Test {
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}

	fn level_init(&mut self, map_name: &CStr) {
		let _ = map_name;
		for class in self.dll.server_classes() {
			let table = class.table();
			con_msg!("{:?} ({:?}), {} SendProp(s)", class.network_name(), table.name(), table.n_props());
			for prop in table.props() {
				con_msg!("    {:?} @{} ({:?})", prop.name(), prop.offset(), prop.prop_type());
			}
		}
	}
}

export_loadable_plugin!(Test);

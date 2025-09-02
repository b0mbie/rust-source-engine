use ::anyhow::{
	Result, Error,
};
use ::rse_server_plugin::prelude::*;
use ::rse_tier0_print::{
	tier0::prelude::*,
	prelude::*,
};

macro_rules! println {
	($t:expr) => {{
		::rse_tier0_print::Printer::print(
			&::rse_tier0_print::tier0::linked::con(),
			::rse_tier0_print::ComposeNewlined::newlined($t),
		)
	}};
}

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
		println!(
			"Test plugin".rgb::<255, 0, 191>()
				.then(" is ".plain())
				.then("unloading".rgb::<255, 0, 0>())
		);
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
			println!(
				class.network_name().plain()
					.then(" (".plain())
					.then(table.name().plain())
					.then("), ".plain())
					.then(table.n_props().plain())
					.then(" SendProp(s)".plain())
			);
			for prop in table.props() {
				use ::rse_server_plugin::game::SendPropType as Pt;
				let pt = match prop.prop_type() {
					Pt::Int => "Int",
					Pt::Float => "Float",
					Pt::Vector => "Vector",
					Pt::VectorXy => "VectorXy",
					Pt::String => "String",
					Pt::Array => "Array",
					Pt::DataTable => "DataTable",
				};
				println!(
					"    ".plain()
						.then(prop.name().plain())
						.then(" @".plain()).then(prop.offset().plain())
						.then(" (".plain()).then(pt.plain()).then(")".plain())
				);
			}
		}
	}
}

export_loadable_plugin!(Test);

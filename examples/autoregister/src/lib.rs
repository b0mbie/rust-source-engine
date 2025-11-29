use rse_std::prelude::*;
use std::convert::Infallible;

autoregistered! {
	static NAME: ConVar = unsafe { ConVar::simple(c"rse_autoregister_name", cvar_value!("Mike")) };
	static O_COUNT: TypedConVar<u8> = unsafe { TypedConVar::new(ConVarParams {
		name: c"rse_autoregister_o_count",
		default: cvar_value!(1),
		..ConVarParams::EMPTY
	}) };

	static PRINT_O_COUNT: ConCommand = ConCommand::new(
		c"rse_autoregister_print_o_count",
		None,
		CvarFlags::empty(),
		move |_| {
			con_msg!("{}", O_COUNT.get());
		},
		None,
	);

	static GREET: ConCommand = ConCommand::new(
		c"rse_autoregister_greet",
		None,
		CvarFlags::empty(),
		move |_| {
			con().msg_raw(c"Hell");
			for _ in 1..=O_COUNT.get() {
				con().msg_raw('o');
			}
			con().msg_raw(c", ");
			con().msg_raw(&*NAME.c_str());
			con().msg_raw(c"!\n");
		},
		None,
	);
}

impl Plugin for AutoRegister {
	type LoadError = Infallible;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}

struct AutoRegister;
export_plugin!(AutoRegister);

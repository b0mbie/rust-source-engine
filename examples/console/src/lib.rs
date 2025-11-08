use rse_std::prelude::*;
use std::convert::Infallible;

static GREETING: ConVar = unsafe { ConVar::new(
	ConVarParams {
		name: c"rse_console_greeting",
		default: cvar_value!("Hello!"),
		help: Some(c"Greeting to prepend when echoing with `rse_console_polite_echo`."),
		..ConVarParams::EMPTY
	}
) };

static GREET: ConCommand = ConCommand::new(
	c"rse_console_polite_echo",
	Some(c"Echoes arguments with politeness."),
	CvarFlags::empty(),
	move |cmd| {
		con().msg_raw(&*GREETING.c_str());
		con().msg_raw(c" ");
		for arg in cmd.iter() {
			con().msg_raw(arg);
			con().msg_raw(c" ");
		}
		con().msg_raw(c"\n");
	},
	None,
);

struct Console;
impl Plugin for Console {
	type LoadError = Infallible;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
		GREETING.register();
		GREET.register();
		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_plugin!(Console);

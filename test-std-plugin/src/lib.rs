use ::anyhow::Error;
use ::rse_std::prelude::*;
use ::std::borrow::Cow;

#[con_var(
	name = "rse_greeting",
	help = "Specifies the greeting to be printed by `rse_greet`.",
)]
static GREETING: Cow<'static, CStr> = "Hello!";

#[con_command(
	name = "rse_greet",
	help = "Print a greeting."
)]
fn greet(_: &Invocation) {
	con().msg(GREETING.get().as_ref());
	con().msg('\n');
}

struct TestStd;

impl Plugin for TestStd {
	type LoadError = Error;
	fn load(factories: InterfaceFactories<'_>) -> Result<Self, Self::LoadError> {
		let _ = factories;
		GREETING.register();
		greet.register();
		Ok(Self)
	}

	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}

export_plugin_as! {
	PLUGIN: TestStd;
}

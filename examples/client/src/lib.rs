use rse_std::prelude::*;

struct FileSystem;
impl Plugin for FileSystem {
	type LoadError = &'static str;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
        
		let (w, h) = rse_std::cl::screen_size();
        con_msg!("Current screen size: {w}x{h}");
		con_msg!();

        con_msg!("Protocol version: {}", rse_std::cl::protocol_version());
        con_msg!("Client version: {}", rse_std::cl::client_version());
		con_msg!();

		rse_std::cl::execute_unrestricted(c"echo Getting real version information with `version`...;version");

		// This should fail because `version` is not marked with a special flag.
		rse_std::cl::execute(c"version");

		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_plugin!(FileSystem);

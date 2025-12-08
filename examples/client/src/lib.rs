use rse_std::prelude::*;
use rse_std::cl;

struct FileSystem;
impl Plugin for FileSystem {
	type LoadError = &'static str;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
        
		let (w, h) = cl::screen_size();
        con_msg!("Current screen size: {w}x{h}");
		con_msg!();

		let con = rse_std::io::con();
		con.msg_raw("Protocol version");
		if let Some(protocol) = cl::protocol_version() {
			con.msg(format_args!("{protocol}\n"));
		} else {
			con.msg_raw(" unknown\n")
		}
		con.msg_raw("Client version");
		if let Some(client) = cl::client_version() {
			con.msg(format_args!("{client}\n"));
		} else {
			con.msg_raw(" unknown\n")
		}
		con.msg_raw('\n');

		cl::execute_unrestricted(c"echo Getting real version information with `version`...;version");

		// This should fail because `version` is not marked with a special flag.
		cl::execute(c"version");

		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_plugin!(FileSystem);

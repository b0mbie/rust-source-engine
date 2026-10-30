use rse_plugin::prelude::*;
use rse_server::interfaces::{
	OServerGameDll, UserMessageInfo,
};
use rse_tier0::prelude::*;
use std::fmt;

struct UserMessageListing;
impl LoadablePlugin for UserMessageListing {
	fn load(factories: PluginFactories) -> Option<Self> {
		let dll = factories.create_interface::<OServerGameDll>().ok()?;

		let mut name_buf = [0; 64];
		con_msg!("UserMessages:");
		for index in 0.. {
			let Some(UserMessageInfo { size }) = dll.get_user_message_info(index, &mut name_buf) else { break };
			let name = CStr::from_bytes_until_nul(&name_buf).ok()?;
			con_msg!("{index}. {name:?} ({})", UserMessageSize(size));
		}

		Some(Self)
	}
}
impl Plugin for UserMessageListing {
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_loadable_plugin!(UserMessageListing);

struct UserMessageSize(pub Option<usize>);
impl fmt::Display for UserMessageSize {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(size) = self.0 {
			size.fmt(f)?;
			f.write_str("B")
		} else {
			f.write_str("variable size")
		}
	}
}

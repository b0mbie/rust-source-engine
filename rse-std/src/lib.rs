pub use ::rse_convar;
pub use ::rse_plugin;

#[cfg(feature = "macros")]
pub use ::rse_std_macros::{
	con_var, con_command,
};

#[doc(hidden)]
pub use ::rse_std_macros::cvar_value_detail;

#[cfg(feature = "macros")]
#[macro_export]
macro_rules! cvar_value {
	($value:literal $(,)?) => {{
		use $crate::var::ConVarValue;
		let _ = $value;
		$crate::cvar_value_detail!($value)
	}};
}

pub(crate) mod futex;
pub(crate) mod c_buffer;
pub(crate) mod c_strings;

pub(crate) mod panicking;
pub(crate) mod threads;

mod macros;

mod console;
pub use console::*;

pub mod alloc;
pub mod env;
pub mod io;
pub mod interfaces;
pub mod plugin;

#[cfg(feature = "server")]
pub mod server;

pub mod prelude {
	pub use ::core::ffi::CStr;
	pub use crate::{
		cmd::{
			Suggestions, Invocation,
			Command, ConCommand,
		},
		var::{
			Variable, OldValue, NewValue,
			ConVar,
		},
		io::{
			con, dev,
			dev_msg, dev_warn, con_msg, con_warn,
		},
		interfaces::InterfaceFactories,
		plugin::{
			ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
			RejectReason, ClientConnect,
			Plugin, plugin_description,
		},
		export_plugin_as, export_plugin,
	};
	#[cfg(feature = "macros")]
	pub use crate::{
		cvar_value,
		con_var, con_command,
	};
}

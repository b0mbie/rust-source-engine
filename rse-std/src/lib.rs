#![warn(
	clippy::alloc_instead_of_core,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
)]

extern crate alloc as rust_alloc;

#[doc(hidden)]
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
		use $crate::con::var::ConVarValue;
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

pub mod alloc;
pub mod con;
pub mod env;
pub mod ffi;
pub mod io;
pub mod interfaces;
pub mod plugin;

pub(crate) mod fs_consts;
#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "sv")]
pub mod sv;

pub mod prelude {
	pub use crate::{
		con::{
			cmd::{
				Suggestions, Invocation,
				Command, ConCommand,
			},
			var::{
				Variable, OldValue, NewValue,
				ConVar, ConVarParams,
			},
			CvarFlags,
		},
		ffi::{
			CStr, CString,
		},
		io::{
			con, dev,
			dev_msg, dev_warn, con_msg, con_warn,
		},
		plugin::{
			ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
			RejectReason, ClientConnect,
			Plugin, plugin_description,
			PluginFactories,
		},
		export_plugin_as, export_plugin,
	};
	#[cfg(feature = "macros")]
	pub use crate::{
		cvar_value,
		con_var, con_command,
	};
}

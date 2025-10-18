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
		use $crate::console::var::ConVarValue;
		let _ = $value;
		$crate::cvar_value_detail!($value)
	}};
}

pub(crate) mod futex;
pub(crate) mod c_buffer;
pub(crate) mod c_strings;

mod macros;

pub mod console;
pub mod interfaces;
pub mod plugin;

pub mod prelude {
	pub use ::core::ffi::CStr;
	pub use ::rse_tier0::{
		linked::{
			cmd::command_line,
			spew::{
				con, dev,
			},
		},
		Tier0CommandLine,
		Logger, ColorLogger,
		dev_msg, dev_warn, con_msg, con_warn,
		msg, msgln, warn, warnln,
	};
	pub use crate::{
		console::{
			cmd::{
				Suggestions, Invocation,
				Command, ConCommand,
			},
			var::{
				Variable, OldValue, NewValue,
				ConVar, cvar_value,
			},
		},
		/*
		cvar::{
			fcvar,
			CvarImpl,
			init as cvar_init,
		},
		*/
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
		con_var, con_command,
	};
}

pub use ::rse_shared::ServerEdict;
pub use ::rse_plugin::{
	cppdef::{
		ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
	},
	RejectReason, ClientConnect,
	plugin_description,
};

mod adapter;
pub use adapter::*;
mod plugin_trait;
pub use plugin_trait::*;

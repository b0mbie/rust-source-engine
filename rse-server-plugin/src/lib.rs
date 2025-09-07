#![no_std]

pub use ::rse_game as game;
pub use ::rse_game_interfaces as game_interfaces;
pub use ::rse_interface as interface;

pub mod cppdef;

mod macros;

mod static_plugin;
pub use static_plugin::*;
mod plugin;
pub use plugin::*;
mod loadable_plugin;
pub use loadable_plugin::*;
mod reject_reason;
pub use reject_reason::*;

pub mod prelude {
	pub use ::core::ffi::CStr;
	pub use crate::{
		cppdef::{
			ClientIndex, PluginResult,
			QueryCvarCookie, QueryCvarValueStatus,
		},
		game::{
			Command, ServerEdict,
		},
		game_interfaces::{
			GameEventManager2, GameEventManager2Impl as _,
			VEngineServer, VEngineServerImpl as _,
			ServerGameDll, ServerGameDllImpl as _,
			InterfaceFactories,
			EmitSound,
		},
		interface::{
			InterfaceFactory, RawInterfaceFactory,
		},
		StaticPlugin, LoadablePlugin, Plugin,
		ClientConnect, RejectReason,
		export_static_plugin, export_loadable_plugin,
		export_static_plugin_as, export_loadable_plugin_as,
		plugin_description,
	};
}

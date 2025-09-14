//! Crate that allows for implementing Source 1 plugins with `no_std` in safe Rust.
//! 
//! Users of this crate will usually want to implement [`LoadablePlugin`]/[`StaticPlugin`] and [`Plugin`].
//! 
//! # Panicking
//! Rust plugin methods that are called by C++ are defined with the `extern "C"` ABI,
//! so panicking inside of such a function will typically not unwind the stack beyond the boundary between C++ and Rust.
//! 
//! **While it is safe to panic, it is not recommended for proper error handling.**
//! Typically, a panic will result in the main process being aborted,
//! with no further information presented to a user of the plugin.
//! Instead of panicking, users of this crate are encouraged to use error handling with [`Result`],
//! where any errors are printed to the console to be presented to users of the plugin.

#![no_std]

pub use ::rse_convar as convar;
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
		convar::Command,
		cppdef::{
			ClientIndex, PluginResult,
			QueryCvarCookie, QueryCvarValueStatus,
		},
		game::ServerEdict,
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

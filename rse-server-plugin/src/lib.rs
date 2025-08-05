#![no_std]

pub use ::rse_game as game;
pub use ::rse_game_interfaces as game_interfaces;
pub use ::rse_interface as interface;

pub mod cppdef;

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
			InterfaceFactories,
			EmitSound,
		},
		interface::{
			InterfaceFactory, RawInterfaceFactory,
		},
		StaticPlugin, LoadablePlugin, Plugin,
		ClientConnect, RejectReason,
		export_static_plugin, export_loadable_plugin,
	};
}

#[macro_export]
macro_rules! export_static_plugin {
	($ty:ty) => {
		const _: () = {
			use ::core::option::Option;
			use $crate::{
				interface::cppdef::ReturnCode,
				PluginObject
			};

			static mut PLUGIN: PluginObject<$ty> = PluginObject::new(<$ty as $crate::StaticPlugin>::NOT_LOADED);

			struct ExportedPlugin;
			impl $crate::interface::RawInterfaceFactory for ExportedPlugin {
				#[allow(static_mut_refs)]
				unsafe fn create_interface_raw(
					&self, name: &::core::ffi::CStr, return_code: Option<&mut ReturnCode>,
				) -> Option<$crate::interface::cppdef::RawInterface> {
					let result = if name == <PluginObject<$ty> as $crate::interface::Interface>::IDENTIFIER {
						unsafe { Some($crate::interface::ToRawInterface::to_raw_interface(&mut PLUGIN)) }
					} else {
						None
					};
					if let Option::Some(return_code) = return_code {
						*return_code = if result.is_some() { ReturnCode::OK } else { ReturnCode::FAILED };
					}
					result
				}
			}
			impl $crate::interface::DllInterfaceFactory for ExportedPlugin {
				const INSTANCE: &Self = &ExportedPlugin;
			}

			$crate::interface::dll_interface_factory!(ExportedPlugin);
		};
	};
}

#[macro_export]
macro_rules! export_loadable_plugin {
	($ty:ty) => {
		$crate::export_static_plugin!($crate::PluginLoader<$ty>);
	};
}

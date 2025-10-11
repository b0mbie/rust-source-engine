use ::core::{
	any::type_name,
	ffi::CStr,
	hint::unreachable_unchecked,
	mem::replace,
};
use ::rse_convar::command::Invocation;
use ::rse_game::ServerEdict;
use ::rse_game_interfaces::InterfaceFactories;

use crate::{
	cppdef::{
		ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
	},
	RejectReason, ClientConnect,
	StaticPlugin, Plugin,
};

/// Helper for a [`LoadablePlugin`] that implements [`StaticPlugin`],
/// which allows for exporting the plugin.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PluginLoader<P> {
	inner: PluginLoaderInner<P>,
}

#[derive(Debug, Clone, Copy)]
enum PluginLoaderInner<P> {
	NotLoaded,
	Loaded(P),
	LoadedAgain(P),
}

impl<P> PluginLoader<P> {
	pub const fn new() -> Self {
		Self {
			inner: PluginLoaderInner::NotLoaded,
		}
	}

	pub const fn plugin(&self) -> Option<&P> {
		match self.inner {
			PluginLoaderInner::NotLoaded => None,
			PluginLoaderInner::Loaded(ref p) | PluginLoaderInner::LoadedAgain(ref p) => Some(p),
		}
	}

	pub const fn plugin_mut(&mut self) -> Option<&mut P> {
		match self.inner {
			PluginLoaderInner::NotLoaded => None,
			PluginLoaderInner::Loaded(ref mut p) | PluginLoaderInner::LoadedAgain(ref mut p) => Some(p),
		}
	}

	const unsafe fn plugin_mut_unchecked(&mut self) -> &mut P {
		match self.inner {
			PluginLoaderInner::NotLoaded => unsafe { unreachable_unchecked() },
			PluginLoaderInner::Loaded(ref mut p) | PluginLoaderInner::LoadedAgain(ref mut p) => p,
		}
	}
}

impl<P> Default for PluginLoader<P> {
	fn default() -> Self {
		Self::new()
	}
}

/// Trait for plugins the instances of which are always initialized when loading,
/// and then cleaned up with [`Drop`] when unloading.
/// 
/// See [`Plugin`] for functionality that can be implemented,
/// and [`StaticPlugin`](crate::StaticPlugin) for an advanced version.
/// 
/// # Errors
/// There is no native way to report an error message to the plugin loader.
/// Consider using the `tier0` library for printing errors to the console.
/// 
/// # Panicking
/// See the [crate-level documentation](crate#panicking) for information about panicking in plugin functions.
pub trait LoadablePlugin: Sized + Plugin {
	/// Returns the initialized plugin,
	/// or `None` if loading failed for whatever reason.
	fn load(factories: InterfaceFactories<'_>) -> Option<Self>;
}

impl<P: LoadablePlugin> StaticPlugin for PluginLoader<P> {
	const NOT_LOADED: Self = Self::new();
	unsafe fn load(&mut self, factories: InterfaceFactories<'_>) -> bool {
		match replace(&mut self.inner, PluginLoaderInner::NotLoaded) {
			PluginLoaderInner::NotLoaded => {
				match P::load(factories) {
					Some(p) => {
						self.inner = PluginLoaderInner::Loaded(p);
						true
					}
					None => false,
				}
			}
			PluginLoaderInner::Loaded(p) => {
				self.inner = PluginLoaderInner::LoadedAgain(p);
				false
			}
			inner => {
				self.inner = inner;

				// This check is here in debug mode to catch a theoretical buggy plugin loader implementation.
				if cfg!(any(test, debug_assertions)) {
					panic!("`PluginLoader<{}>` loaded again without unloading first", type_name::<P>());
				} else {
					unsafe { unreachable_unchecked() }
				}
			}
		}
	}
	unsafe fn unload(&mut self) {
		match replace(&mut self.inner, PluginLoaderInner::NotLoaded) {
			PluginLoaderInner::NotLoaded => { /* nothing to do */ }
			PluginLoaderInner::Loaded(p) => {
				drop(p);
				self.inner = PluginLoaderInner::NotLoaded;
			}
			PluginLoaderInner::LoadedAgain(p) => {
				self.inner = PluginLoaderInner::Loaded(p);
			}
		}
	}
}

macro_rules! delegates {
	{
		$(fn $name:ident(&mut self $(, $param:ident: $param_ty:ty)* $(,)?) $(-> $return:ty)?;)*
	} => {
		$(fn $name(&mut self $(, $param: $param_ty)*) $(-> $return)? {
			unsafe { self.plugin_mut_unchecked().$name($($param,)*) }
		})*
	};
}

impl<P: Plugin> Plugin for PluginLoader<P> {
	delegates! {
		fn pause(&mut self);
		fn unpause(&mut self);
		fn description(&mut self) -> &CStr;
		fn level_init(&mut self, map_name: &CStr);
		fn server_activate(&mut self, edicts: &mut [ServerEdict], client_max: ClientIndex);
		fn game_frame(&mut self, simulating: bool);
		fn level_shutdown(&mut self);
		fn client_active(&mut self, entity: &mut ServerEdict);
		fn client_disconnect(&mut self, entity: &mut ServerEdict);
		fn client_put_in_server(&mut self, entity: &mut ServerEdict, player_name: &CStr);
		fn set_command_client(&mut self, index: ClientIndex);
		fn client_settings_changed(&mut self, edict: &mut ServerEdict);
		fn client_connect(
			&mut self,
			entity: &mut ServerEdict,
			name: &CStr, address: &CStr,
			reject_reason: RejectReason<'_>,
		) -> ClientConnect;
		fn client_command(&mut self, entity: &mut ServerEdict, invocation: &Invocation) -> PluginResult;
		fn network_id_validated(&mut self, user_name: &CStr, network_id: &CStr) -> PluginResult;
		fn on_query_cvar_value_finished(
			&mut self,
			cookie: QueryCvarCookie,
			player_entity: &mut ServerEdict,
			status: QueryCvarValueStatus,
			cvar_name: &CStr, cvar_value: &CStr,
		);
		fn on_edict_allocated(&mut self, edict: &mut ServerEdict);
		fn on_edict_freed(&mut self, edict: &ServerEdict);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct Dummy;
	impl LoadablePlugin for Dummy {
		fn load(factories: InterfaceFactories<'_>) -> Option<Self> {
			let _ = factories;
			Some(Self)
		}
	}
	impl Plugin for Dummy {
		fn description(&mut self) -> &CStr {
			c"Dummy"
		}
	}
	
	#[test]
	#[should_panic(expected = "loaded again without unloading first")]
	fn plugin_loader_no_multiple_loads() {
		let mut loader = PluginLoader::<Dummy>::new();
		unsafe {
			unsafe extern "C" fn factrie(
				_: *const ::core::ffi::c_char, _: *mut ::rse_interface::ReturnCode,
			) -> Option<::core::ptr::NonNull<::core::ffi::c_void>> {
				None
			}
			assert!(loader.load(InterfaceFactories::new(factrie, factrie)));
			assert!(!loader.load(InterfaceFactories::new(factrie, factrie)));
			assert!(!loader.load(InterfaceFactories::new(factrie, factrie)));
		}
	}
}

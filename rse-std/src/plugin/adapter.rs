use ::core::{
	any::type_name,
	ffi::CStr,
	hint::unreachable_unchecked,
	mem::replace,
};
use ::rse_game_interfaces::{
	InterfaceFactories,
};
use ::rse_plugin::{
	StaticPlugin, Plugin as CPlugin,
};
use ::rse_tier0::con_warn;

use crate::cmd::Invocation;

use super::{
	ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
	RejectReason, ClientConnect,
	ServerEdict,
	Plugin,
};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Adapter<P> {
	inner: Inner<P>,
}

impl<P> Default for Adapter<P> {
	fn default() -> Self {
		Self::new()
	}
}

impl<P> Adapter<P> {
	pub const fn new() -> Self {
		Self {
			inner: Inner::NotLoaded,
		}
	}

	pub const fn plugin(&self) -> Option<&P> {
		match self.inner {
			Inner::NotLoaded => None,
			Inner::Loaded(ref p) | Inner::LoadedAgain(ref p) => Some(p),
		}
	}

	pub const fn plugin_mut(&mut self) -> Option<&mut P> {
		match self.inner {
			Inner::NotLoaded => None,
			Inner::Loaded(ref mut p) | Inner::LoadedAgain(ref mut p) => Some(p),
		}
	}

	const unsafe fn plugin_mut_unchecked(&mut self) -> &mut P {
		match self.inner {
			Inner::NotLoaded => unsafe { unreachable_unchecked() },
			Inner::Loaded(ref mut p) | Inner::LoadedAgain(ref mut p) => p,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Inner<P> {
	NotLoaded,
	Loaded(P),
	LoadedAgain(P),
}

impl<P> StaticPlugin for Adapter<P>
where
	P: Plugin,
{
	unsafe fn load(&mut self, factories: InterfaceFactories<'_>) -> bool {
		crate::threads::MAIN_THREAD.bind_to_current();

		macro_rules! init {
			($result:expr $(,)?) => {
				if !$result {
					return false
				}
			};
		}

		// SAFETY: `Self::load` is called on the main thread; `detach` is called in `Self::unload`.
		unsafe { crate::cvar::attach(factories) }

		#[cfg(feature = "server")]
		unsafe { init!(crate::server::attach(factories)) }

		match replace(&mut self.inner, Inner::NotLoaded) {
			Inner::NotLoaded => {
				match P::load(factories) {
					Ok(plugin) => {
						self.inner = Inner::Loaded(plugin);
						true
					}
					Err(error) => {
						con_warn!("{error}");
						false
					}
				}
			}
			Inner::Loaded(p) => {
				self.inner = Inner::LoadedAgain(p);
				false
			}
			inner => {
				self.inner = inner;

				// This check is here in debug mode to catch a theoretical buggy plugin loader implementation.
				if cfg!(any(test, debug_assertions)) {
					panic!("`Adapter<{}>` loaded again without unloading first", type_name::<P>());
				} else {
					unsafe { unreachable_unchecked() }
				}
			}
		}
	}
	unsafe fn unload(&mut self) {
		match replace(&mut self.inner, Inner::NotLoaded) {
			Inner::NotLoaded => { /* nothing to do */ }
			Inner::Loaded(p) => {
				// SAFETY: `Self::unload` is called on the main thread.
				unsafe { crate::cvar::detach() };
				drop(p);
				self.inner = Inner::NotLoaded;
			}
			Inner::LoadedAgain(p) => {
				self.inner = Inner::Loaded(p);
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

impl<P> CPlugin for Adapter<P>
where
	P: Plugin,
{
	delegates! {
		fn pause(&mut self);
		fn unpause(&mut self);
		fn description(&mut self) -> &CStr;
		fn level_init(&mut self, map_name: &CStr);
		fn server_activate(&mut self, edicts: &mut [ServerEdict], client_max: ClientIndex);
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
	
	fn game_frame(&mut self, simulating: bool) {
		unsafe { self.plugin_mut_unchecked().game_frame(simulating) }
	}
}

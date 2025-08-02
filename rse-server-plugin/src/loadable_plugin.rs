use ::core::ffi::CStr;
use ::rse_game::{
	Command, ServerEdict,
	InterfaceFactories,
};

use crate::{
	cppdef::{
		ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
	},
	RejectReason, ClientConnect,
	DllPlugin, Plugin,
};

pub struct PluginLoader<P> {
	plugin: Option<P>,
	had_failed_load: bool,
}

impl<P> PluginLoader<P> {
	const unsafe fn as_plugin_mut_unchecked(&mut self) -> &mut P {
		unsafe { self.plugin.as_mut().unwrap_unchecked() }
	}
}

impl<P> Default for PluginLoader<P> {
	fn default() -> Self {
		Self::new()
	}
}

impl<P> PluginLoader<P> {
	pub const fn new() -> Self {
		Self {
			plugin: None,
			had_failed_load: false,
		}
	}
}

pub trait LoadablePlugin: Sized + Plugin {
	fn load(factories: InterfaceFactories<'_>) -> Option<Self>;
}

impl<P: LoadablePlugin> DllPlugin for PluginLoader<P> {
	const NOT_LOADED: Self = Self::new();
	fn load(&mut self, factories: InterfaceFactories<'_>) -> bool {
		if self.plugin.is_none() {
			self.plugin = P::load(factories);
			self.plugin.is_some()
		} else {
			self.had_failed_load = true;
			false
		}
	}
	fn unload(&mut self) {
		if self.had_failed_load {
			self.had_failed_load = false;
		} else {
			self.plugin = None;
		}
	}
}

macro_rules! delegates {
	{
		$(fn $name:ident(&mut self $(, $param:ident: $param_ty:ty)* $(,)?) $(-> $return:ty)?;)*
	} => {
		$(fn $name(&mut self $(, $param: $param_ty)*) $(-> $return)? {
			unsafe { self.as_plugin_mut_unchecked().$name($($param,)*) }
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
		fn client_command(&mut self, entity: &mut ServerEdict, command: &Command) -> PluginResult;
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

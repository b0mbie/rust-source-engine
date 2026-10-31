use ::core::ffi::CStr;

pub use ::rse_convar::command::Invocation;
pub use ::rse_shared::ServerEdict;

use crate::{
	cppdef::{
		ClientIndex, PluginResult, QueryCvarValueStatus,
	},
	QueryCvarCookie,
	RejectReason,
};

/// Trait for defining common functionality in plugins.
/// 
/// See [`LoadablePlugin`](crate::LoadablePlugin) or [`StaticPlugin`](crate::StaticPlugin)
/// for implementations of plugin loading.
/// 
/// # Default implementations
/// The trait provides a default, no-op implementation for *most* functions.
/// They can be freely overriden by plugin implementations.
/// 
/// # Panicking
/// See the [crate-level documentation](crate#panicking) for information about panicking in plugin functions.
pub trait Plugin {
	/// Returns the human-readable description - the name - of the plugin.
	/// 
	/// This function must be manually implemented by all plugins - there is no default implementation.
	/// Consider using [`plugin_description!`](crate::plugin_description!)
	/// to generate a usable string that represents the plugin,
	/// or use C string literal syntax (e.g. `c"Plugin v0.1.0"`) to safely create a static C string.
	fn description(&mut self) -> &CStr;

	fn pause(&mut self) {}
	fn unpause(&mut self) {}
	fn level_init(&mut self, map_name: &CStr) {
		let _ = map_name;
	}
	fn server_activate(&mut self, edicts: &mut [ServerEdict], client_max: ClientIndex) {
		let _ = edicts;
		let _ = client_max;
	}
	fn game_frame(&mut self, simulating: bool) {
		let _ = simulating;
	}
	fn level_shutdown(&mut self) {}
	fn client_active(&mut self, entity: &mut ServerEdict) {
		let _ = entity;
	}
	fn client_disconnect(&mut self, entity: &mut ServerEdict) {
		let _ = entity;
	}
	fn client_put_in_server(&mut self, entity: &mut ServerEdict, player_name: &CStr) {
		let _ = entity;
		let _ = player_name;
	}
	fn set_command_client(&mut self, index: ClientIndex) {
		let _ = index;
	}
	fn client_settings_changed(&mut self, edict: &mut ServerEdict) {
		let _ = edict;
	}
	fn client_connect(
		&mut self,
		entity: &mut ServerEdict,
		name: &CStr, address: &CStr,
		reject_reason: &mut RejectReason,
		allow_connect: &mut bool,
	) -> PluginResult {
		let _ = entity;
		let _ = name;
		let _ = address;
		let _ = reject_reason;
		let _ = allow_connect;
		PluginResult::Continue
	}
	fn client_command(&mut self, entity: &mut ServerEdict, cmd: &Invocation) -> PluginResult {
		let _ = entity;
		let _ = cmd;
		PluginResult::Continue
	}
	fn network_id_validated(&mut self, user_name: &CStr, network_id: &CStr) -> PluginResult {
		let _ = user_name;
		let _ = network_id;
		PluginResult::Continue
	}
	fn query_cvar_value_finished(
		&mut self,
		cookie: QueryCvarCookie,
		player_entity: &mut ServerEdict,
		status: QueryCvarValueStatus,
		cvar_name: &CStr, cvar_value: &CStr,
	) {
		let _ = cookie;
		let _ = player_entity;
		let _ = status;
		let _ = cvar_name;
		let _ = cvar_value;
	}
	fn edict_allocated(&mut self, edict: &mut ServerEdict) {
		let _ = edict;
	}
	fn edict_freed(&mut self, edict: &ServerEdict) {
		let _ = edict;
	}
}

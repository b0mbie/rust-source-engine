use ::core::{
	ffi::CStr,
	fmt::Display,
};

use crate::{
	con::cmd::Invocation,
	plugin::PluginFactories,
};

use super::{
	ClientIndex, PluginResult, QueryCvarCookie, QueryCvarValueStatus,
	RejectReason, ClientConnect,
	ServerEdict,
};

pub trait Plugin: Sized {
	// TODO: Better error bound?
	// `anyhow::Error` nor `Box<dyn ::core::error::Error>` work with `::core::error::Error`,
	// which could otherwise provide nice logging for errors.
	/// Type for errors that can occur during [`load`](Plugin::load).
	type LoadError: Display;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError>;

	fn description(&mut self) -> &CStr;

	/// Called when the plugin is attempted to be loaded a second time.
	fn repeated_load(&mut self, factories: PluginFactories) {
		let _ = factories;
	}

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
		reject_reason: RejectReason<'_>,
	) -> ClientConnect {
		let _ = entity;
		let _ = name;
		let _ = address;
		let _ = reject_reason;
		ClientConnect::Allow
	}
	fn client_command(&mut self, entity: &mut ServerEdict, invocation: &Invocation) -> PluginResult {
		let _ = entity;
		let _ = invocation;
		PluginResult::Continue
	}
	fn network_id_validated(&mut self, user_name: &CStr, network_id: &CStr) -> PluginResult {
		let _ = user_name;
		let _ = network_id;
		PluginResult::Continue
	}
	fn on_query_cvar_value_finished(
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
	fn on_edict_allocated(&mut self, edict: &mut ServerEdict) {
		let _ = edict;
	}
	fn on_edict_freed(&mut self, edict: &ServerEdict) {
		let _ = edict;
	}
}

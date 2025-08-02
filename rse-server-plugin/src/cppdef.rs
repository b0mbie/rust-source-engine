use ::core::ffi::{
	CStr, c_char, c_int,
};
use ::rse_cpp::{
	RefConst, VtObjectMut, vtable,
};
use ::rse_interface::cppdef::CreateInterfaceFn;
use ::rse_game::cppdef::{
	convar::Command,
	entities::Edict,
	KeyValues,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum PluginResult {
	/// Keep going.
	Continue = 0,
	/// Run the game DLL function, but use our return value instead.
	Override,
	/// Don't run the game DLL function at all.
	Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum QueryCvarValueStatus {
	/// Got the value fine.
	ValueIntact = 0,
	/// There is no ConVar or ConCommand with the given name.
	CvarNotFound = 1,
	/// There's a ConCommand, but it's not a ConVar.
	NotACvar = 2,
	/// The cvar was marked with `FCVAR_SERVER_CAN_NOT_QUERY`,
	/// so the server is not allowed to have its value.
	CvarProtected = 3,
}

pub type QueryCvarCookie = c_int;
pub const INVALID_QUERY_CVAR_COOKIE: QueryCvarCookie = -1;

pub const INTERFACEVERSION_ISERVERPLUGINCALLBACKS: &CStr = c"ISERVERPLUGINCALLBACKS003";

pub type ClientIndex = c_int;

vtable! {
	pub ServerPluginCallbacksVt {
		pub fn load(interface_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> bool;
		pub fn unload();
		pub fn pause();
		pub fn unpause();
		pub fn get_plugin_description() -> *const c_char;
		pub fn level_init(map_name: *const c_char);
		pub fn server_activate(edict_list: *mut Edict, edict_count: c_int, client_max: ClientIndex);
		pub fn game_frame(simulating: bool);
		pub fn level_shutdown();
		pub fn client_active(entity: *mut Edict);
		pub fn client_disconnect(entity: *mut Edict);
		pub fn client_put_in_server(entity: *mut Edict, player_name: *const c_char);
		pub fn set_command_client(index: ClientIndex);
		pub fn client_settings_changed(edict: *mut Edict);
		pub fn client_connect(
			out_allow_connect: *mut bool,
			entity: *mut Edict,
			name: *const c_char, address: *const c_char,
			out_reject: *mut c_char, out_reject_len: c_int,
		) -> PluginResult;
		pub fn client_command(entity: *mut Edict, args: RefConst<Command>) -> PluginResult;
		pub fn network_id_validated(user_name: *const c_char, network_id: *const c_char) -> PluginResult;
		pub fn on_query_cvar_value_finished(
			cookie: QueryCvarCookie,
			player_entity: *mut Edict,
			status: QueryCvarValueStatus,
			cvar_name: *const c_char, cvar_value: *const c_char,
		);
		pub fn on_edict_allocated(edict: *mut Edict);
		pub fn on_edict_freed(edict: *const Edict);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum DialogType {
	/// Just an on-screen message.
	Msg = 0,
	/// An options menu.
	Menu,
	/// A RichText dialog.
	Text,
	/// An entry box.
	Entry,
	/// Ask the client to connect to a specified IP address.
	/// 
	/// Only the `time` and `title` keys are used.
	AskConnect,
}

vtable! {
	pub ServerPluginHelpersVt {
		pub fn create_message(
			entity: *mut Edict,
			dialog_type: DialogType, data: *mut KeyValues,
			plugin: VtObjectMut<ServerPluginCallbacksVt>,
		);
		pub fn client_command(entity: *mut Edict, cmd: *const c_char);
		pub fn start_query_cvar_value(entity: *mut Edict, name: *const c_char) -> QueryCvarCookie;
	}
}

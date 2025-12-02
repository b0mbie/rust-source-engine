use ::core::ffi::{
	CStr, c_char, c_float, c_int,
};
use ::rse_shared::cppdef::{
	entities::{
		edict_t, ServerClass,
	},
	GlobalVars,
};
use ::rse_interface::CreateInterfaceFn;

pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_8: &CStr = c"ServerGameDLL008";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_9: &CStr = c"ServerGameDLL009";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_10: &CStr = c"ServerGameDLL010";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_11: &CStr = c"ServerGameDLL011";
pub const INTERFACEVERSION_SERVERGAMEDLL: &CStr = c"ServerGameDLL012";
pub const INTERFACEVERSION_SERVERGAMEDLL_INT: c_int = 12;

pub type TickInterval = c_float;

::rse_cpp::vtable! {
	pub ServerGameDllVt {
		pub fn dll_init(
			engine_factory: CreateInterfaceFn, physics_factory: CreateInterfaceFn, file_system_factory: CreateInterfaceFn,
			globals: *mut GlobalVars,
		) -> bool;
		pub fn replay_init(replay_factory: CreateInterfaceFn) -> bool;
		pub fn game_init() -> bool;
		pub fn level_init(
			map_name: *const c_char, map_entities: *const c_char,
			old_level: *const c_char, landmark_name: *const c_char,
			load_game: bool, background: bool,
		) -> bool;
		pub fn server_activate(edict_list: *mut edict_t, edict_count: c_int, client_max: c_int);
		pub fn game_frame(simulating: bool);
		pub fn pre_client_update(simulating: bool);
		pub fn level_shutdown();
		pub fn game_shutdown();
		pub fn dll_shutdown();
		pub fn get_tick_interval() -> TickInterval;
		pub fn get_all_server_classes() -> *mut ServerClass;
		pub fn get_game_description() -> *const c_char;
		pub fn create_network_string_tables();
		
		// TODO: `SaveInit` and everything else after it.

		// TODO: Figure out what the difference is between different `ServerGameDLL` versions.
	}
}

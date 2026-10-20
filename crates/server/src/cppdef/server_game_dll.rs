use ::core::ffi::{
	CStr, c_char, c_float, c_int, c_uchar,
};
use ::rse_convar::cppdef::CCommand;
use ::rse_cpp::{
	RefConst, RefMut,
	class,
};
use ::rse_interface::CreateInterfaceFn;
use ::rse_math::Vector;
use ::rse_shared::cppdef::{
	entities::{
		edict_t, ServerClass,
	},
	CGlobalVars,
	KeyValues,
	bf_read,
};

pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_8: &CStr = c"ServerGameDLL008";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_9: &CStr = c"ServerGameDLL009";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_10: &CStr = c"ServerGameDLL010";
pub const INTERFACEVERSION_SERVERGAMEDLL_VERSION_11: &CStr = c"ServerGameDLL011";
pub const INTERFACEVERSION_SERVERGAMEDLL: &CStr = c"ServerGameDLL012";
pub const INTERFACEVERSION_SERVERGAMEDLL_INT: c_int = 12;

pub type TickInterval = c_float;

class! {
	pub IServerGameDLL {
		pub virtual fn dll_init(
			engine_factory: CreateInterfaceFn, physics_factory: CreateInterfaceFn, file_system_factory: CreateInterfaceFn,
			globals: *mut CGlobalVars,
		) -> bool;
		pub virtual fn replay_init(replay_factory: CreateInterfaceFn) -> bool;
		pub virtual fn game_init() -> bool;
		pub virtual fn level_init(
			map_name: *const c_char, map_entities: *const c_char,
			old_level: *const c_char, landmark_name: *const c_char,
			load_game: bool, background: bool,
		) -> bool;
		pub virtual fn server_activate(edict_list: *mut edict_t, edict_count: c_int, client_max: c_int);
		pub virtual fn game_frame(simulating: bool);
		pub virtual fn pre_client_update(simulating: bool);
		pub virtual fn level_shutdown();
		pub virtual fn game_shutdown();
		pub virtual fn dll_shutdown();
		pub virtual fn get_tick_interval() -> TickInterval;
		pub virtual fn get_all_server_classes() -> *mut ServerClass;
		pub virtual fn get_game_description() -> *const c_char;
		pub virtual fn create_network_string_tables();
		
		// TODO: `SaveInit` and everything else after it.

		// TODO: Figure out what the difference is between different `ServerGameDLL` versions.
	}
}

pub const INTERFACEVERSION_SERVERGAMECLIENTS_VERSION_3: &CStr = c"ServerGameClients003";
pub const INTERFACEVERSION_SERVERGAMECLIENTS: &CStr = c"ServerGameClients004";

class! {
	pub IServerGameClients {
		pub virtual fn get_player_limits(
			min_players: RefMut<c_int>, max_players: RefMut<c_int>,
			default_max_players: RefMut<c_int>,
		);
		pub virtual fn client_connect(
			entity: *mut edict_t,
			name: *const c_char, address: *const c_char,
			reject: *mut c_char, max_reject_len: c_int,
		) -> bool;
		pub virtual fn client_active(entity: *mut edict_t, load_game: bool);
		pub virtual fn client_disconnect(entity: *mut edict_t);
		pub virtual fn client_put_in_server(entity: *mut edict_t, player_name: *const c_char);
		pub virtual fn client_command(entity: *mut edict_t, args: RefConst<CCommand>);
		pub virtual fn set_command_client(index: c_int);
		pub virtual fn client_settings_changed(edict: *mut edict_t);
		pub virtual fn client_setup_visibility(
			view_entity: *mut edict_t, client: *mut edict_t,
			pvs: *const c_uchar, pvs_size: c_int,
		);
		pub virtual fn process_usercmds(
			player: *mut edict_t, buf: *mut bf_read,
			num_cmds: c_int, total_cmds: c_int,
			dropped_packets: c_int,
			ignore: bool, paused: bool,
		) -> c_float;
		pub virtual fn post_client_messsages_sent_deprecated();
		pub virtual fn get_player_state(player: *mut edict_t) -> *mut CPlayerState;
		pub virtual fn client_ear_position(entity: *mut edict_t, ear_origin: *mut Vector);
		pub virtual fn get_replay_delay(player: *mut edict_t, entity: RefMut<c_int>) -> c_int;
		pub virtual fn get_bug_report_info(buf: *mut c_char, buf_len: c_int);
		pub virtual fn network_id_validated(user_name: *const c_char, network_id: *const c_char);
		pub virtual fn client_command_key_values(entity: *mut edict_t, key_values: *mut KeyValues);
		pub virtual fn client_spawned(player: *mut edict_t);
	}
}

class! {
	// TODO: `CPlayerState`.
	pub CPlayerState {}
}

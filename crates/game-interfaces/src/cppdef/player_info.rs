use ::core::ffi::{
	CStr, c_char, c_float, c_int, c_short,
};
use ::rse_cpp::{
	class,
	RefConst, RefMut,
};
use ::rse_shared::cppdef::{
	entities::edict_t,
	CGlobalVars,
};
use ::rse_math::{
	Vector, QAngle,
};

pub const INTERFACEVERSION_PLAYERINFOMANAGER: &CStr = c"PlayerInfoManager002";

class! {
	pub IPlayerInfoManager {
		pub virtual fn get_player_info(edict: *mut edict_t) -> *mut IPlayerInfo;
		pub virtual fn get_global_vars() -> *mut CGlobalVars;
	}
}

class! {
	pub IPlayerInfo {
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn get_user_id() -> c_int;
		pub virtual fn get_network_id_string() -> *const c_char;
		pub virtual fn get_team_index() -> c_int;
		pub virtual fn change_team(team_num: c_int);
		pub virtual fn get_frag_count() -> c_int;
		pub virtual fn get_death_count() -> c_int;
		pub virtual fn is_connected() -> bool;
		pub virtual fn get_armor_value() -> c_int;
	}
}

class! {
	pub IPlayerInfo_V2: pub virtual IPlayerInfo {
		pub virtual fn is_hltv() -> bool;
		pub virtual fn is_player() -> bool;
		pub virtual fn is_fake_client() -> bool;
		pub virtual fn is_dead() -> bool;
		pub virtual fn is_in_a_vehicle() -> bool;
		pub virtual fn is_observer() -> bool;

		pub virtual fn get_abs_origin() -> Vector;
		pub virtual fn get_abs_angles() -> QAngle;
		pub virtual fn get_player_mins() -> Vector;
		pub virtual fn get_player_maxs() -> Vector;
		pub virtual fn get_weapon_name() -> *const c_char;
		pub virtual fn get_model_name() -> *const c_char;
		pub virtual fn get_health() -> c_int;
		pub virtual fn get_max_health() -> c_int;
		pub virtual fn get_last_user_command() -> CBotCmd;

		pub virtual fn is_replay() -> bool;
	}
}

class! {
	pub CBotCmd {
		pub virtual ~destructors();

		/// Number for matching server and client commands for debugging.
		pub command_number: c_int;
		/// Tick the client created this command.
		pub tick_count: c_int;
		/// Instantaneous view angles.
		pub view_angles: QAngle;
		/// Intended forward velocity.
		/// 
		/// Positive values indicate movement forwards,
		/// negative values indicate movement backwards.
		pub forward_move: c_float;
		/// Intended sideways velocity.
		/// 
		/// Positive values indicate movement to the right,
		/// negative values indicate movement to the left.
		pub side_move: c_float;
		/// Intended upward velocity.
		/// 
		/// Positive values indicate movement upwards,
		/// negative values indicate movement downwards.
		pub up_move: c_float;
		/// Bit-set of button states.
		pub buttons: c_int;
		/// Impulse command issued,
		/// or `0` if no command was issued.
		pub impulse: u8;
		/// Entity index of the weapon that was selected.
		pub weapon_select: c_int;
		/// Sub-type of the weapon that was selected.
		pub weapon_subtype: c_int;
		/// Seed for shared random functions.
		pub random_seed: c_int;
		/// Mouse movement on the X axis.
		/// 
		/// Positive values indicate movement to the right,
		/// negative values indicate movement to the left.
		pub mouse_dx: c_short;
		/// Mouse movement on the Y axis.
		/// 
		/// Positive values indicate movement upwards,
		/// negative values indicate movement downwards.
		pub mouse_dy: c_short;
		/// **Client-only:**
		/// `true` if this command was predicted at least once.
		pub has_been_predicted: bool;
	}
}

class! {
	pub IBotController {
		pub virtual fn set_abs_origin(vec: RefMut<Vector>);
		pub virtual fn set_abs_angles(ang: RefMut<Vector>);
		pub virtual fn set_local_origin(origin: RefConst<Vector>);
		pub virtual fn get_local_origin() -> Vector;
		pub virtual fn set_local_angles(angles: RefConst<QAngle>);
		pub virtual fn get_local_angles() -> QAngle;

		pub virtual fn remove_all_items(remove_suit: bool);
		pub virtual fn set_active_weapon(weapon_name: *const c_char);
		pub virtual fn is_eflag_set(eflag_mask: c_int) -> bool;
		pub virtual fn run_player_move(ucmd: *mut CBotCmd);
	}
}

pub const INTERFACEVERSION_PLAYERBOTMANAGER: &CStr = c"BotManager001";

class! {
	pub IBotManager {
		pub virtual fn get_bot_controller(edict: *mut edict_t) -> *mut IBotController;
		pub virtual fn create_bot(bot_name: *const c_char) -> *mut edict_t;
	}
}

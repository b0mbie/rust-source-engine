use ::core::{
	ffi::{
		CStr, c_char, c_float, c_int, c_short,
	},
	ptr::NonNull,
};
use ::rse_cpp::{
	vtable, VtObjectMut, VtObjectPtr,
	RefConst, RefMut,
};
use ::rse_game::cppdef::{
	entities::edict_t,
	GlobalVars,
};
use ::rse_math::{
	Vector, QAngle,
};

pub const INTERFACEVERSION_PLAYERINFOMANAGER: &CStr = c"PlayerInfoManager002";

vtable! {
	pub PlayerInfoManagerVt {
		pub fn get_player_info(edict: *mut edict_t) -> Option<VtObjectMut<PlayerInfo2Vt>>;
		pub fn get_global_vars() -> *mut GlobalVars;
	}
}

vtable! {
	pub PlayerInfoVt {
		pub fn get_name() -> *const c_char;
		pub fn get_user_id() -> c_int;
		pub fn get_network_id_string() -> *const c_char;
		pub fn get_team_index() -> c_int;
		pub fn change_team(team_num: c_int);
		pub fn get_frag_count() -> c_int;
		pub fn get_death_count() -> c_int;
		pub fn is_connected() -> bool;
		pub fn get_armor_value() -> c_int;
	}
}

#[repr(C)]
pub struct PlayerInfo2Vt {
	pub v1: PlayerInfoVt,
	pub v2: PlayerInfo2VtBase,
}

vtable! {
	pub PlayerInfo2VtBase for VtObjectPtr<PlayerInfo2Vt> {
		pub fn is_hltv() -> bool;
		pub fn is_player() -> bool;
		pub fn is_fake_client() -> bool;
		pub fn is_dead() -> bool;
		pub fn is_in_a_vehicle() -> bool;
		pub fn is_observer() -> bool;

		pub fn get_abs_origin() -> Vector;
		pub fn get_abs_angles() -> QAngle;
		pub fn get_player_mins() -> Vector;
		pub fn get_player_maxs() -> Vector;
		pub fn get_weapon_name() -> *const c_char;
		pub fn get_model_name() -> *const c_char;
		pub fn get_health() -> c_int;
		pub fn get_max_health() -> c_int;
		pub fn get_last_user_command() -> BotCmd;

		pub fn is_replay() -> bool;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct BotCmd {
	/// VTable for `CBotCmd`.
	pub vtable: NonNull<BotCmdVt>,
	/// Number for matching server and client commands for debugging.
	pub command_number: c_int,
	/// Tick the client created this command.
	pub tick_count: c_int,
	/// Instantaneous view angles.
	pub view_angles: QAngle,
	/// Intended forward velocity.
	/// 
	/// Positive values indicate movement forwards,
	/// negative values indicate movement backwards.
	pub forward_move: c_float,
	/// Intended sideways velocity.
	/// 
	/// Positive values indicate movement to the right,
	/// negative values indicate movement to the left.
	pub side_move: c_float,
	/// Intended upward velocity.
	/// 
	/// Positive values indicate movement upwards,
	/// negative values indicate movement downwards.
	pub up_move: c_float,
	/// Bit-set of button states.
	pub buttons: c_int,
	/// Impulse command issued,
	/// or `0` if no command was issued.
	pub impulse: u8,
	/// Entity index of the weapon that was selected.
	pub weapon_select: c_int,
	/// Sub-type of the weapon that was selected.
	pub weapon_subtype: c_int,
	/// Seed for shared random functions.
	pub random_seed: c_int,
	/// Mouse movement on the X axis.
	/// 
	/// Positive values indicate movement to the right,
	/// negative values indicate movement to the left.
	pub mouse_dx: c_short,
	/// Mouse movement on the Y axis.
	/// 
	/// Positive values indicate movement upwards,
	/// negative values indicate movement downwards.
	pub mouse_dy: c_short,
	/// **Client-only:**
	/// `true` if this command was predicted at least once.
	pub has_been_predicted: bool,
}

impl BotCmd {
	pub const VTABLE: &BotCmdVt = &::rse_cpp::new_vtable_self!(BotCmdVt {
		destructor,
		#[cfg(not(windows))]
		destructor_2
	});

	::rse_cpp::vtable_methods! {
		this: VtObjectPtr<BotCmdVt>;
		fn destructor() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		#[cfg(not(windows))]
		fn destructor_2() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
	}
}

vtable! {
	pub BotCmdVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
	}
}

vtable! {
	pub BotControllerVt {
		pub fn set_abs_origin(vec: RefMut<Vector>);
		pub fn set_abs_angles(ang: RefMut<Vector>);
		pub fn set_local_origin(origin: RefConst<Vector>);
		pub fn get_local_origin() -> Vector;
		pub fn set_local_angles(angles: RefConst<QAngle>);
		pub fn get_local_angles() -> QAngle;

		pub fn remove_all_items(remove_suit: bool);
		pub fn set_active_weapon(weapon_name: *const c_char);
		pub fn is_eflag_set(eflag_mask: c_int) -> bool;
		pub fn run_player_move(ucmd: *mut BotCmd);
	}
}

pub const INTERFACEVERSION_PLAYERBOTMANAGER: &CStr = c"BotManager001";

vtable! {
	pub BotManagerVt {
		pub fn get_bot_controller(edict: *mut edict_t) -> Option<VtObjectMut<BotControllerVt>>;
		pub fn create_bot(bot_name: *const c_char) -> *mut edict_t;
	}
}

use ::core::ffi::{
	CStr, c_char, c_float, c_int,
};
use ::rse_cpp::{
	RefConst, RefMut,
	class,
};
use ::rse_math::{
	Vector, QAngle,
};
use ::rse_shared::cppdef::entities::{
	IServerUnknown, edict_t,
};

use super::pluginvariant;

class! {
	pub IEntityInfo {
		pub virtual fn entity_index() -> c_int;
		pub virtual fn get_entity_name() -> *const c_char;
		pub virtual fn get_classname() -> *const c_char;
		pub virtual fn get_model_name() -> *const c_char;
		pub virtual fn get_target_name() -> *const c_char;
		pub virtual fn set_model(model_name: *const c_char);
		pub virtual fn is_player() -> bool;
		pub virtual fn is_npc() -> bool;
		pub virtual fn is_dead() -> bool;
		pub virtual fn is_alive() -> bool;
		pub virtual fn is_in_world() -> bool;
		pub virtual fn is_template() -> bool;
		pub virtual fn get_e_flags() -> c_int;
		pub virtual fn set_e_flags(flags: c_int);
		pub virtual fn add_e_flags(flag_mask: c_int);
		pub virtual fn is_e_flag_set(flag_mask: c_int);

		pub virtual fn get_effects() -> c_int;
		pub virtual fn add_effects(effects: c_int);
		pub virtual fn remove_effects(effects: c_int);
		pub virtual fn clear_effects();
		pub virtual fn set_effects(effects: c_int);
		pub virtual fn is_effect_active(effects: c_int) -> bool;

		pub virtual fn get_render_mode() -> c_int;
		pub virtual fn set_render_mode(mode: c_int);

		pub virtual fn set_blocks_los(blocks_los: bool);
		pub virtual fn blocks_los() -> bool;

		pub virtual fn get_health() -> c_int;
		pub virtual fn get_max_health() -> c_int;
		pub virtual fn set_health(health: c_int);
		pub virtual fn set_max_health(max_health: c_int);

		pub virtual fn get_team_index() -> c_int;
		pub virtual fn change_team(team_num: c_int);

		pub virtual fn get_abs_origin() -> Vector;
		pub virtual fn set_abs_origin(vec: RefMut<Vector>);
		pub virtual fn get_abs_angles() -> QAngle;
		pub virtual fn set_abs_angles(ang: RefMut<QAngle>);
		pub virtual fn get_local_origin() -> Vector;
		pub virtual fn set_local_origin(origin: RefConst<Vector>);
		pub virtual fn get_local_angles() -> QAngle;
		pub virtual fn set_local_angles(angles: RefConst<QAngle>);

		pub virtual fn get_abs_velocity() -> Vector;
		pub virtual fn get_local_velocity() -> Vector;
		pub virtual fn get_local_angular_velocity() -> QAngle;

		pub virtual fn entity_to_world_space(input: RefConst<Vector>, output: *mut Vector);
		pub virtual fn world_to_entity_space(input: RefConst<Vector>, output: *mut Vector);

		pub virtual fn eye_position() -> Vector;
		pub virtual fn eye_angles() -> QAngle;
		pub virtual fn local_eye_angles() -> QAngle;
		pub virtual fn ear_position() -> Vector;

		pub virtual fn get_world_mins() -> Vector;
		pub virtual fn get_world_maxs() -> Vector;
		pub virtual fn world_space_center() -> Vector;

		pub virtual fn get_water_level() -> c_int;

		pub virtual fn get_owner() -> *mut edict_t;
		pub virtual fn get_parent() -> *mut edict_t;
		pub virtual fn get_move_parent() -> *mut edict_t;
		pub virtual fn get_root_move_parent() -> *mut edict_t;

		pub virtual fn get_followed_entity() -> *mut edict_t;
		pub virtual fn get_ground_entity() -> *mut edict_t;

		pub virtual fn get_custom_info(
			value_type: c_int, out_value: RefMut<pluginvariant>,
			options: pluginvariant,
		) -> bool;

		pub virtual fn get_debug_name() -> *const c_char;
		pub virtual fn entity_text(
			text_offset: c_int,
			text: *const c_char,
			duration: c_float,
			r: c_int, g: c_int, b: c_int, a: c_int,
		);

		pub virtual fn get_key_value(key_name: *const c_char, value: *mut c_char, max_len: c_int) -> bool;
	}
}

class! {
	pub IEntityInfoManager {
		pub virtual fn get_entity_info(edict: *mut edict_t) -> *mut IEntityInfo;
		pub virtual fn get_entity_info_by_index(index: c_int) -> *mut IEntityInfo;

		pub virtual fn get_server_entity(edict: *mut edict_t) -> *mut IServerUnknown;

		pub virtual fn find_entity_by_classname(start_entity: *mut edict_t, name: *const c_char) -> *mut edict_t;
		pub virtual fn find_entity_by_name(start_entity: *mut edict_t, name: *const c_char) -> *mut edict_t;
		pub virtual fn find_entity_by_model(start_entity: *mut edict_t, model_name: *const c_char) -> *mut edict_t;
		pub virtual fn find_entity_in_sphere(
			start_entity: *mut edict_t,
			center: RefConst<Vector>, radius: c_float,
		) -> *mut edict_t;

		pub virtual fn get_world_bounds(mins: RefMut<Vector>, maxs: RefMut<Vector>);
	}
}

pub const INTERFACEVERSION_ENTITYINFOMANAGER: &CStr = c"EntityInfoManager001";

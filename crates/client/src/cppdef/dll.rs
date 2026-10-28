use ::core::ffi::{
	CStr, c_char, c_float, c_int,
};
use ::rse_cpp::{
	class,
	RefMut, RefConst,
};
use ::rse_interface::cppdef::CreateInterfaceFn;
use ::rse_math::{
	Vector, QAngle, VMatrix,
};
use ::rse_shared::cppdef::{
	CGlobalVarsBase,
	bf_read, bf_write,
	ScreenFade_t,
};
use ::rse_tier0::{
	vrect_t, qboolean,
};

use super::{
	entities::ClientClass,
	ButtonCode_t,
};

class! {
	pub IBaseClientDLL {
		pub virtual fn init(
			app_system_factory: CreateInterfaceFn,
			physics_factory: CreateInterfaceFn,
			globals: *mut CGlobalVarsBase,
		);
		pub virtual fn post_init();
		pub virtual fn shutdown();
		pub virtual fn replay_init(replay_factory: CreateInterfaceFn);
		pub virtual fn replay_post_init();
		pub virtual fn level_init_pre_entity(map_name: *const c_char);
		pub virtual fn level_init_post_entity();
		pub virtual fn level_shutdown();
		pub virtual fn get_all_classes() -> *mut ClientClass;
		pub virtual fn hud_vid_init() -> c_int;
		pub virtual fn hud_process_input(active: bool);
		pub virtual fn hud_update(active: bool);
		pub virtual fn hud_reset();
		pub virtual fn hud_text(message: *const c_char);
		pub virtual fn in_activate_mouse();
		pub virtual fn in_deactivate_mouse();
		pub virtual fn in_accumulate();
		pub virtual fn in_clear_states();
		pub virtual fn in_is_key_down(name: *const c_char, is_down: RefMut<bool>) -> bool;
		pub virtual fn in_on_mouse_wheeled(delta: c_int);
		pub virtual fn in_key_event(event_code: c_int, key_num: ButtonCode_t, current_binding: *const c_char) -> c_int;
		pub virtual fn create_move(sequence_number: c_int, input_sample_frametime: c_float, active: bool);
		pub virtual fn extra_mouse_sample(frame_time: c_float, active: bool);
		pub virtual fn write_usercmd_delta_to_buffer(
			buf: *mut bf_write,
			from: c_int, to: c_int,
			is_new_command: bool,
		) -> bool;
		pub virtual fn encode_user_cmd_to_buffer(buf: RefMut<bf_write>, slot: c_int);
		pub virtual fn decode_user_cmd_to_buffer(buf: RefMut<bf_read>, slot: c_int);
		pub virtual fn view_render(rect: *mut vrect_t);
		pub virtual fn render_view(view: RefConst<CViewSetup>, clear_flags: c_int, what_to_draw: c_int);
		pub virtual fn view_fade(sf: *mut ScreenFade_t);
		pub virtual fn set_crosshair_angle(angle: RefConst<QAngle>);
		pub virtual fn init_sprite(sprite: *mut CEngineSprite, load_name: *const c_char);
		pub virtual fn shutdown_sprite(sprite: *mut CEngineSprite);
		pub virtual fn get_sprite_size() -> c_int;
		pub virtual fn voice_status(ent_index: c_int, talking: qboolean) -> c_int;
		pub virtual fn install_string_table_callback(table_name: *const c_char);
		pub virtual fn frame_stage_notify(cur_stage: ClientFrameStage_t);
		pub virtual fn dispatch_user_message(msg_type: c_int, msg_data: RefConst<bf_read>);
		// TODO: `SaveInit` and all others after it.
	}
}

pub const CLIENT_DLL_INTERFACE_VERSION: &CStr = c"VClient017";

class! {
	// TODO: `CEngineSprite`.
	pub CEngineSprite {}
}

class! {
	pub CViewSetup {
		pub x: c_int;
		pub unscaled_x: c_int;
		pub y: c_int;
		pub unscaled_y: c_int;
		pub width: c_int;
		pub unscaled_width: c_int;
		pub height: c_int;
		pub stereo_eye: StereoEye_t;
		pub unscaled_height: c_int;
		pub ortho: bool;
		pub ortho_left: c_float;
		pub ortho_top: c_float;
		pub ortho_right: c_float;
		pub ortho_bottom: c_float;
		pub fov: c_float;
		pub fov_viewmodel: c_float;
		pub origin: Vector;
		pub angles: QAngle;
		pub z_near: c_float;
		pub z_far: c_float;
		pub z_near_viewmodel: c_float;
		pub z_far_viewmodel: c_float;
		pub render_to_subrect_of_larger_screen: bool;
		pub aspect_ratio: c_float;
		pub off_center: bool;
		pub off_center_top: c_float;
		pub off_center_bottom: c_float;
		pub off_center_left: c_float;
		pub off_center_right: c_float;
		pub do_bloom_and_tone_mapping: bool;
		pub cache_full_scene_state: bool;
		pub view_to_projection_override: bool;
		pub view_to_projection: VMatrix;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum ClientFrameStage_t {
	Undefined = -1,
	Start,
	NetUpdateStart,
	NetUpdatePostDataUpdateStart,
	NetUpdatePostDataUpdateEnd,
	NetUpdateEnd,
	RenderStart,
	RenderEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum StereoEye_t {
	Mono = 0,
	Left = 1,
	Right = 2,
	Max = 3,
}

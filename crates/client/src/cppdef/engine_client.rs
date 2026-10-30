use ::core::ffi::{
	CStr, c_char, c_int, c_float, c_void, c_uint,
};
use ::rse_cpp::{
	RefConst, RefMut,
	class,
};
use ::rse_game_interfaces::cppdef::AudioState_t;
use ::rse_shared::cppdef::{
	player_info_t, client_textmessage_t, con_nprint_t, model_t,
	INetChannelInfo,
	IAchievementMgr, CGamestatsData,
	KeyValues,
};
use ::rse_math::{
	SurfInfo, Vector, QAngle, VMatrix, matrix3x4_t,
	color32,
};
use ::rse_sound::cppdef::{
	CSentence, CAudioSource,
};

use super::{
	IMaterial, IMaterialSystem,
	ButtonCode_t,
};

pub const VENGINE_CLIENT_INTERFACE_VERSION: &CStr = c"VEngineClient014";
pub const VENGINE_CLIENT_INTERFACE_VERSION_13: &CStr = c"VEngineClient013";

class! {
	pub IVEngineClient: pub virtual IVEngineClient013 {
		pub virtual fn get_protocol_version() -> c_uint;
		pub virtual fn is_windowed_mode() -> bool;
		pub virtual fn flash_window();
		pub virtual fn get_client_version() -> c_int;
		pub virtual fn is_active_app() -> bool;
		pub virtual fn disconnect_internal();
		pub virtual fn get_instances_running_count() -> c_int;
		pub virtual fn set_rich_presence_connect(override_str: *const c_char);
	}
}

class! {
	// TODO: This should be tested to see if any functions were missed.
	pub IVEngineClient013 {
		pub virtual fn get_intersecting_surfaces(
			model: *const model_t,
			center: RefConst<Vector>, radius: c_float,
			only_visible_surfaces: bool,
			infos: *mut SurfInfo, max_infos: c_int,
		) -> c_int;
		pub virtual fn get_light_for_point(pos: RefConst<Vector>, clamp: bool) -> Vector;
		pub virtual fn trace_line_material_and_lighting(
			start: RefConst<Vector>, end: RefConst<Vector>,
			diffuse_light_color: RefMut<Vector>,
			base_color: RefMut<Vector>,
		) -> *mut IMaterial;
		pub virtual fn parse_file(data: *const c_char, token: *mut c_char, max_len: c_int) -> *const c_char;
		pub virtual fn copy_local_file(source: *const c_char, destination: *const c_char) -> bool;
		pub virtual fn get_screen_size(width: RefMut<c_int>, height: RefMut<c_int>);
		pub virtual fn server_cmd(cmd_string: *const c_char, reliable: bool);
		pub virtual fn client_cmd(cmd_string: *const c_char);
		pub virtual fn get_player_info(ent_num: c_int, pinfo: *mut player_info_t) -> bool;
		pub virtual fn get_player_for_user_id(user_id: c_int) -> c_int;
		pub virtual fn text_message_get(name: *const c_char) -> *mut client_textmessage_t;
		pub virtual fn con_is_visible() -> bool;
		pub virtual fn get_local_player() -> c_int;
		pub virtual fn load_model(name: *const c_char, prop: bool) -> *const model_t;
		pub virtual fn time() -> c_float;
		pub virtual fn get_last_time_stamp() -> c_float;
		pub virtual fn get_sentence(audio_source: *mut CAudioSource) -> *mut CSentence;
		pub virtual fn get_sentence_length(audio_source: *mut CAudioSource) -> c_float;
		pub virtual fn is_streaming(audio_source: *mut CAudioSource) -> bool;
		pub virtual fn get_view_angles(va: RefMut<QAngle>);
		pub virtual fn set_view_angles(va: RefMut<QAngle>);
		pub virtual fn get_max_clients() -> c_int;
		pub virtual fn key_lookup_binding(binding: *const c_char) -> *const c_char;
		pub virtual fn key_binding_for_key(code: ButtonCode_t) -> *const c_char;
		pub virtual fn start_key_trap_mode();
		pub virtual fn check_done_key_trapping(code: RefMut<ButtonCode_t>) -> bool;
		pub virtual fn is_in_game() -> bool;
		pub virtual fn is_connected() -> bool;
		pub virtual fn is_drawing_loading_image() -> bool;
		pub virtual fn con_n_printf(pos: c_int, fmt: *const c_char, ...) -> bool;
		pub virtual fn con_nx_printf(info: *const con_nprint_t, fmt: *const c_char, ...) -> bool;
		pub virtual fn is_box_visible(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> c_int;
		pub virtual fn is_box_in_view_cluster(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> c_int;
		pub virtual fn cull_box(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> bool;
		pub virtual fn sound_extra_update();
		pub virtual fn get_game_directory() -> *const c_char;
		pub virtual fn world_to_screen_matrix() -> RefConst<VMatrix>;
		pub virtual fn world_to_view_matrix() -> RefConst<VMatrix>;
		pub virtual fn game_lump_version(lump_id: c_int) -> c_int;
		pub virtual fn game_lump_size(lump_id: c_int) -> c_int;
		pub virtual fn load_game_lump(lump_id: c_int, buffer: *mut c_void, size: c_int) -> bool;
		pub virtual fn level_leaf_count() -> c_int;
		// TODO: Nullable?
		pub virtual fn get_bsp_tree_query() -> *mut ISpatialQuery;
		pub virtual fn linear_to_gamma(linear: *mut c_float, gamma: *mut c_float);
		pub virtual fn light_style_value(style: c_int) -> c_float;
		pub virtual fn compute_dynamic_lighting(pt: RefConst<Vector>, normal: *const Vector, color: RefMut<Vector>);
		pub virtual fn get_ambient_light_color(color: RefMut<Vector>);
		pub virtual fn get_dx_support_level() -> c_int;
		pub virtual fn supports_hdr() -> bool;
		pub virtual fn mat_stub(mat_sys: *mut IMaterialSystem);
		pub virtual fn get_chapter_name(buffer: *mut c_char, max_length: c_int);
		pub virtual fn get_level_name() -> *const c_char;
		pub virtual fn get_level_version() -> c_int;
		#[cfg(feature = "voice")]
		pub virtual fn get_voice_tweak_api() -> *mut IVoiceTweak_s;
		pub virtual fn engine_stats_begin_frame();
		pub virtual fn engine_stats_end_frame();
		pub virtual fn fire_events();
		pub virtual fn get_leaves_area(leaves: *mut c_int, n_leaves: c_int) -> c_int;
		pub virtual fn does_box_touch_area_frustum(mins: RefConst<Vector>, maxs: RefConst<Vector>, area: c_int) -> bool;
		pub virtual fn set_audio_state(state: RefConst<AudioState_t>);
		pub virtual fn sentence_group_pick(group_index: c_int, name: *mut c_char, name_len: c_int) -> c_int;
		pub virtual fn sentence_group_pick_sequential(
			group_index: c_int,
			name: *mut c_char, name_len: c_int,
			sentence_index: c_int,
			reset: c_int,
		) -> c_int;
		pub virtual fn sentence_index_from_name(sentence_name: *const c_char) -> c_int;
		pub virtual fn sentence_name_from_index(sentence_index: c_int) -> *const c_char;
		pub virtual fn sentence_group_index_from_name(name: *const c_char) -> c_int;
		pub virtual fn sentence_group_name_from_index(group_index: c_int) -> *const c_char;
		pub virtual fn sentence_length(sentence_index: c_int) -> c_float;
		pub virtual fn compute_lighting(
			pt: RefConst<Vector>, normal: *const Vector,
			clamp: bool,
			color: RefMut<Vector>,
			box_colors: *mut Vector,
		);
		pub virtual fn activate_occluder(occluder_index: c_int, active: bool);
		pub virtual fn is_occluded(abs_mins: RefConst<Vector>, abs_maxs: RefConst<Vector>) -> bool;
		pub virtual fn save_alloc_memory(num: usize, size: usize) -> *mut c_void;
		pub virtual fn save_free_memory(save_mem: *mut c_void);
		pub virtual fn get_net_channel_info() -> *mut INetChannelInfo;
		pub virtual fn debug_draw_phys_collide(
			collide: *const CPhysCollide, material: *mut IMaterial,
			transform: RefConst<matrix3x4_t>,
			color: RefConst<color32>,
		);
		pub virtual fn check_point(name: *const c_char);
		pub virtual fn draw_portals();
		pub virtual fn is_playing_demo() -> bool;
		pub virtual fn is_recording_demo() -> bool;
		pub virtual fn is_playing_time_demo() -> bool;
		pub virtual fn get_demo_recording_tick() -> c_int;
		pub virtual fn get_demo_playback_tick() -> c_int;
		pub virtual fn get_demo_playback_start_tick() -> c_int;
		pub virtual fn get_demo_playback_time_scale() -> c_float;
		pub virtual fn get_demo_playback_total_ticks() -> c_int;
		pub virtual fn is_paused() -> bool;
		pub virtual fn is_taking_screenshot() -> bool;
		pub virtual fn is_hltv() -> bool;
		pub virtual fn is_level_main_menu_background() -> bool;
		pub virtual fn get_main_menu_background_name(dest: *mut c_char, dest_len: c_int);
		// FIXME: `vmode_s *&pModes`?
		pub virtual fn get_video_modes(count: RefMut<c_int>, modes: RefMut<*mut vmode_s>);
		pub virtual fn set_occlusion_parameters(params: RefConst<OcclusionParams_t>);
		pub virtual fn get_ui_language(dest: *mut c_char, dest_len: c_int);
		pub virtual fn is_skybox_visible_from_point(point: RefConst<Vector>) -> SkyboxVisibility;
		pub virtual fn get_map_entities_string() -> *const c_char;
		pub virtual fn is_in_edit_mode() -> bool;
		pub virtual fn get_screen_aspect_ratio() -> c_float;
		pub virtual fn removed_steam_refresh_login();
		pub virtual fn removed_steam_process_call();
		pub virtual fn get_engine_build_number() -> c_uint;
		pub virtual fn get_product_version_string() -> *const c_char;
		pub virtual fn grab_pre_color_corrected_frame(x: c_int, y: c_int, width: c_int, height: c_int);
		pub virtual fn is_hammer_running() -> bool;
		pub virtual fn execute_client_cmd(cmd_string: *const c_char);
		pub virtual fn map_has_hdr_lighting() -> bool;
		pub virtual fn get_app_id() -> c_int;
		pub virtual fn get_light_for_point_fast(pos: RefConst<Vector>, clamp: bool) -> Vector;
		pub virtual fn client_cmd_unrestricted(cmd_string: *const c_char);
		pub virtual fn set_restrict_server_commands(restrict: bool);
		pub virtual fn set_restrict_client_commands(restrict: bool);
		pub virtual fn set_overlay_bind_proxy(overlay_id: c_int, bind_proxy: *mut c_void);
		pub virtual fn copy_frame_buffer_to_material(material_name: *const c_char) -> bool;
		pub virtual fn change_team(team_name: *const c_char);
		pub virtual fn read_configuration(read_default: bool);
		pub virtual fn set_achievement_mgr(mgr: *mut IAchievementMgr);
		pub virtual fn get_achievement_mgr() -> *mut IAchievementMgr;
		pub virtual fn map_load_failed() -> bool;
		pub virtual fn set_map_load_failed(state: bool);
		pub virtual fn is_low_violence() -> bool;
		pub virtual fn get_most_recent_save_game() -> *const c_char;
		pub virtual fn set_most_recent_save_game(filename: *const c_char);
		pub virtual fn start_xbox_exiting_process();
		pub virtual fn is_save_in_progress() -> bool;
		pub virtual fn on_storage_device_attached() -> c_uint;
		pub virtual fn on_storage_device_detached();
		pub virtual fn reset_demo_interpolation();
		pub virtual fn set_gamestats_data(gamestats_data: *mut CGamestatsData);
		pub virtual fn get_gamestats_data() -> *mut CGamestatsData;
		#[cfg(all(unix, not(feature = "unix-no-sdl")))]
		pub virtual fn get_mouse_delta(x: RefMut<c_int>, y: RefMut<c_int>, ignore_next_mouse_delta: bool);
		pub virtual fn server_cmd_key_values(key_values: *mut KeyValues);
		pub virtual fn is_skipping_playback() -> bool;
		pub virtual fn is_loading_demo() -> bool;
		pub virtual fn is_playing_demo_a_locally_recorded_demo();
		pub virtual fn key_lookup_binding_exact(binding: *const c_char) -> *const c_char;
		pub virtual fn add_phoneme_file(phoneme_file: *const c_char);
		pub virtual fn get_paused_expire_time() -> c_float;
		pub virtual fn start_demo_recording(filename: *const c_char, folder: *const c_char) -> bool;
		pub virtual fn stop_demo_recording();
		pub virtual fn take_screenshot(filename: *const c_char, folder: *const c_char);
	}
}

class! {
	// TODO: `ISpatialQuery`.
	ISpatialQuery {}
}

// TODO: `IVoiceTweak_s`.
#[repr(C)]
struct IVoiceTweak_s {}

class! {
	// TODO: `CPhysCollide`.
	CPhysCollide {}
}

// TODO: `vmode_s`.
#[repr(C)]
struct vmode_s {}

// TODO: `OcclusionParams_t`.
#[repr(C)]
struct OcclusionParams_t {}

// TODO: `SkyboxVisibility_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SkyboxVisibility {
	What,
}

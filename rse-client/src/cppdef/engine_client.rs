use ::core::ffi::{
	CStr, c_char, c_int, c_float, c_void, c_uint,
};
use ::rse_cpp::{
	RefConst, RefMut,
	VtObjectMut,
	VtObjectPtr,
	vtable,
};
use ::rse_shared::cppdef::{
	player_info_t, client_textmessage_t, con_nprint_t, model_t,
	NetChannelInfoVt,
	AchievementMgrVt, GamestatsData,
	KeyValues,
};
use ::rse_math::{
	SurfInfo, Vector, QAngle, VMatrix, Matrix3x4,
	Color32,
};
use ::rse_sound::cppdef::{
	Sentence, AudioSourceVt,
};

use super::{
	MaterialVt, MaterialSystemVt,
	ButtonCode,
};

pub const VENGINE_CLIENT_INTERFACE_VERSION: &CStr = c"VEngineClient014";
pub const VENGINE_CLIENT_INTERFACE_VERSION_13: &CStr = c"VEngineClient013";

vtable! {
	pub VEngineClientVtBase for VtObjectPtr<VEngineClientVt> {
		pub fn get_protocol_veresion() -> c_uint;
		pub fn is_windowed_mode() -> bool;
		pub fn flash_window();
		pub fn get_client_version() -> c_int;
		pub fn is_active_app() -> bool;
		pub fn disconnect_internal();
		pub fn get_instances_running_count() -> c_int;
		pub fn set_rich_presence_connect(override_str: *const c_char);
	}
}

#[repr(C)]
pub struct VEngineClientVt {
	pub v013: VEngineClient013Vt,
	pub engine_client: VEngineClientVtBase,
}

vtable! {
	// TODO: This should be tested to see if any functions were missed.
	pub VEngineClient013Vt {
		pub fn get_intersecting_surfaces(
			model: *const model_t,
			center: RefConst<Vector>, radius: c_float,
			only_visible_surfaces: bool,
			infos: *mut SurfInfo, max_infos: c_int,
		) -> c_int;
		pub fn get_light_for_point(pos: RefConst<Vector>, clamp: bool) -> Vector;
		// TODO: Nullable?
		pub fn trace_line_material_and_lighting(
			start: RefConst<Vector>, end: RefConst<Vector>,
			diffuse_light_color: RefMut<Vector>,
			base_color: RefMut<Vector>,
		) -> Option<VtObjectMut<MaterialVt>>;
		pub fn parse_file(data: *const c_char, token: *mut c_char, max_len: c_int) -> *const c_char;
		pub fn copy_local_file(source: *const c_char, destination: *const c_char) -> bool;
		pub fn get_screen_size(width: RefMut<c_int>, height: RefMut<c_int>);
		pub fn server_cmd(cmd_string: *const c_char, reliable: bool);
		pub fn client_cmd(cmd_string: *const c_char);
		pub fn get_player_info(ent_num: c_int, pinfo: *mut player_info_t) -> bool;
		pub fn get_player_for_user_id(user_id: c_int) -> c_int;
		pub fn text_message_get(name: *const c_char) -> *mut client_textmessage_t;
		pub fn con_is_visible() -> bool;
		pub fn get_local_player() -> c_int;
		pub fn load_model(name: *const c_char, prop: bool) -> *const model_t;
		pub fn time() -> c_float;
		pub fn get_last_time_stamp() -> c_float;
		pub fn get_sentence(audio_source: VtObjectMut<AudioSourceVt>) -> *mut Sentence;
		pub fn get_sentence_length(audio_source: VtObjectMut<AudioSourceVt>) -> c_float;
		pub fn is_streaming(audio_source: VtObjectMut<AudioSourceVt>) -> bool;
		pub fn get_view_angles(va: RefMut<QAngle>);
		pub fn set_view_angles(va: RefMut<QAngle>);
		pub fn get_max_clients() -> c_int;
		pub fn key_lookup_binding(binding: *const c_char) -> *const c_char;
		pub fn key_binding_for_key(code: ButtonCode) -> *const c_char;
		pub fn start_key_trap_mode();
		pub fn check_done_key_trapping(code: RefMut<ButtonCode>) -> bool;
		pub fn is_in_game() -> bool;
		pub fn is_connected() -> bool;
		pub fn is_drawing_loading_image() -> bool;
		pub fn con_n_printf(pos: c_int, fmt: *const c_char, ...) -> bool;
		pub fn con_nx_printf(info: *const con_nprint_t, fmt: *const c_char, ...) -> bool;
		pub fn is_box_visible(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> c_int;
		pub fn is_box_in_view_cluster(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> c_int;
		pub fn cull_box(mins: RefConst<Vector>, maxs: RefConst<Vector>) -> bool;
		pub fn sound_extra_update();
		pub fn get_game_directory() -> *const c_char;
		pub fn world_to_screen_matrix() -> RefConst<VMatrix>;
		pub fn world_to_view_matrix() -> RefConst<VMatrix>;
		pub fn game_lump_version(lump_id: c_int) -> c_int;
		pub fn game_lump_size(lump_id: c_int) -> c_int;
		pub fn load_game_lump(lump_id: c_int, buffer: *mut c_void, size: c_int) -> bool;
		pub fn level_leaf_count() -> c_int;
		// TODO: Nullable?
		pub fn get_bsp_tree_query() -> Option<VtObjectMut<SpatialQueryVt>>;
		pub fn linear_to_gamma(linear: *mut c_float, gamma: *mut c_float);
		pub fn light_style_value(style: c_int) -> c_float;
		pub fn compute_dynamic_lighting(pt: RefConst<Vector>, normal: *const Vector, color: RefMut<Vector>);
		pub fn get_ambient_light_color(color: RefMut<Vector>);
		pub fn get_dx_support_level() -> c_int;
		pub fn supports_hdr() -> bool;
		// TODO: Nullable?
		pub fn mat_stub(mat_sys: Option<VtObjectMut<MaterialSystemVt>>);
		pub fn get_chapter_name(buffer: *mut c_char, max_length: c_int);
		pub fn get_level_name() -> *const c_char;
		pub fn get_level_version() -> c_int;
		#[cfg(feature = "voice")]
		pub fn get_voice_tweak_api() -> *mut VoiceTweak;
		pub fn engine_stats_begin_frame();
		pub fn engine_stats_end_frame();
		pub fn fire_events();
		pub fn get_leaves_area(leaves: *mut c_int, n_leaves: c_int) -> c_int;
		pub fn does_box_touch_area_frustum(mins: RefConst<Vector>, maxs: RefConst<Vector>, area: c_int) -> bool;
		pub fn set_audio_state(state: RefConst<AudioState>);
		pub fn sentence_group_pick(group_index: c_int, name: *mut c_char, name_len: c_int) -> c_int;
		pub fn sentence_group_pick_sequential(
			group_index: c_int,
			name: *mut c_char, name_len: c_int,
			sentence_index: c_int,
			reset: c_int,
		) -> c_int;
		pub fn sentence_index_from_name(sentence_name: *const c_char) -> c_int;
		pub fn sentence_name_from_index(sentence_index: c_int) -> *const c_char;
		pub fn sentence_group_index_from_name(name: *const c_char) -> c_int;
		pub fn sentence_group_name_from_index(group_index: c_int) -> *const c_char;
		pub fn sentence_length(sentence_index: c_int) -> c_float;
		pub fn compute_lighting(
			pt: RefConst<Vector>, normal: *const Vector,
			clamp: bool,
			color: RefMut<Vector>,
			box_colors: *mut Vector,
		);
		pub fn activate_occluder(occluder_index: c_int, active: bool);
		pub fn is_occluded(abs_mins: RefConst<Vector>, abs_maxs: RefConst<Vector>) -> bool;
		pub fn save_alloc_memory(num: usize, size: usize) -> *mut c_void;
		pub fn save_free_memory(save_mem: *mut c_void);
		// TODO: Nullable?
		pub fn get_net_channel_info() -> Option<VtObjectMut<NetChannelInfoVt>>;
		pub fn debug_draw_phys_collide(
			collide: *const PhysCollide, material: VtObjectMut<MaterialVt>,
			transform: RefConst<Matrix3x4>,
			color: RefConst<Color32>,
		);
		pub fn check_point(name: *const c_char);
		pub fn draw_portals();
		pub fn is_playing_demo() -> bool;
		pub fn is_recording_demo() -> bool;
		pub fn is_playing_time_demo() -> bool;
		pub fn get_demo_recording_tick() -> c_int;
		pub fn get_demo_playback_tick() -> c_int;
		pub fn get_demo_playback_start_tick() -> c_int;
		pub fn get_demo_playback_time_scale() -> c_float;
		pub fn get_demo_playback_total_ticks() -> c_int;
		pub fn is_paused() -> bool;
		pub fn is_taking_screenshot() -> bool;
		pub fn is_hltv() -> bool;
		pub fn is_level_main_menu_background() -> bool;
		pub fn get_main_menu_background_name(dest: *mut c_char, dest_len: c_int);
		// FIXME: `vmode_s *&pModes`?
		pub fn get_video_modes(count: RefMut<c_int>, modes: RefMut<*mut VideoMode>);
		pub fn set_occlusion_parameters(params: RefConst<OcclusionParams>);
		pub fn get_ui_language(dest: *mut c_char, dest_len: c_int);
		pub fn is_skybox_visible_from_point(point: RefConst<Vector>) -> SkyboxVisibility;
		pub fn get_map_entities_string() -> *const c_char;
		pub fn is_in_edit_mode() -> bool;
		pub fn get_screen_aspect_ratio() -> c_float;
		pub fn removed_steam_refresh_login();
		pub fn removed_steam_process_call();
		pub fn get_engine_build_number() -> c_uint;
		pub fn get_product_version_string() -> *const c_char;
		pub fn grab_pre_color_corrected_frame(x: c_int, y: c_int, width: c_int, height: c_int);
		pub fn is_hammer_running() -> bool;
		pub fn execute_client_cmd(cmd_string: *const c_char);
		pub fn map_has_hdr_lighting() -> bool;
		pub fn get_app_id() -> c_int;
		pub fn get_light_for_point_fast(pos: RefConst<Vector>, clamp: bool) -> Vector;
		pub fn client_cmd_unrestricted(cmd_string: *const c_char);
		pub fn set_restrict_server_commands(restrict: bool);
		pub fn set_restrict_client_commands(restrict: bool);
		pub fn set_overlay_bind_proxy(overlay_id: c_int, bind_proxy: *mut c_void);
		pub fn copy_frame_buffer_to_material(material_name: *const c_char) -> bool;
		pub fn change_team(team_name: *const c_char);
		pub fn read_configuration(read_default: bool);
		// TODO: Nullable?
		pub fn set_achievement_mgr(mgr: VtObjectMut<AchievementMgrVt>);
		// TODO: Nullable?
		pub fn get_achievement_mgr() -> Option<VtObjectMut<AchievementMgrVt>>;
		pub fn map_load_failed() -> bool;
		pub fn set_map_load_failed(state: bool);
		pub fn is_low_violence() -> bool;
		pub fn get_most_recent_save_game() -> *const c_char;
		pub fn set_most_recent_save_game(filename: *const c_char);
		pub fn start_xbox_exiting_process();
		pub fn is_save_in_progress() -> bool;
		pub fn on_storage_device_attached() -> c_uint;
		pub fn on_storage_device_detached();
		pub fn reset_demo_interpolation();
		pub fn set_gamestats_data(gamestats_data: *mut GamestatsData);
		pub fn get_gamestats_data() -> *mut GamestatsData;
		#[cfg(feature = "sdl")]
		pub fn get_mouse_delta(x: RefMut<c_int>, y: RefMut<c_int>, ignore_next_mouse_delta: bool);
		pub fn server_cmd_key_values(key_values: *mut KeyValues);
		pub fn is_skipping_playback() -> bool;
		pub fn is_loading_demo() -> bool;
		pub fn is_playing_demo_a_locally_recorded_demo();
		pub fn key_lookup_binding_exact(binding: *const c_char) -> *const c_char;
		pub fn add_phoneme_file(phoneme_file: *const c_char);
		pub fn get_paused_expire_time() -> c_float;
		pub fn start_demo_recording(filename: *const c_char, folder: *const c_char) -> bool;
		pub fn stop_demo_recording();
		pub fn take_screenshot(filename: *const c_char, folder: *const c_char);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct AudioState {
	pub origin: Vector,
	pub angles: QAngle,
	pub is_underwater: bool,
}

vtable! {
	// TODO: `ISpatialQuery`.
	SpatialQueryVt {}
}

// TODO: `IVoiceTweak_s`.
#[repr(C)]
struct VoiceTweak {}

// TODO: `CPhysCollide`.
#[repr(C)]
struct PhysCollide {}

// TODO: `vmode_s`.
#[repr(C)]
struct VideoMode {}

// TODO: `OcclusionParams_t`.
#[repr(C)]
struct OcclusionParams {}

// TODO: `SkyboxVisibility_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SkyboxVisibility {
	What,
}

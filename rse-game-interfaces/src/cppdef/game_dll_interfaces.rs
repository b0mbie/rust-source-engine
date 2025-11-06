use ::core::ffi::{
	CStr, c_char, c_float, c_int, c_long, c_uchar, c_void,
};
use ::rse_cpp::{
	RefConst, RefMut, VtObjectMut, vtable,
};
use ::rse_shared::cppdef::{
	datatable::SendTable,
	entities::{
		edict_t, ServerClass, PvsInfo,
		CollideableVt,
	},
	KeyValues,
	SteamId,
	BfWrite,
	SoundLevel,
};
use ::rse_math::Vector;
use ::rse_scratch_pad::cppdef::ScratchPad3DVt;

// TODO: `INetChannelInfo`.
vtable! {
	pub NetChannelInfoVt {}
}

// TODO: `CBitVec`.
pub type BitVec = c_void;

// TODO: `IRecipientFilter`.
vtable! {
	pub RecipientFilterVt {}
}

// TODO: `struct con_nprint_s`.
pub type ConNPrint = c_void;

// TODO: `VPlane`.
pub type VPlane = c_void;

// TODO: `client_textmessage_t`.
pub type ClientTextMessage = c_void;

// TODO: `ISpatialPartition`.
vtable! {
	pub SpatialPartitionVt {}
}

// TODO: `CCheckTransmitInfo`.
pub type CheckTransmitInfo = c_void;

// TODO: `CSharedEdictChangeInfo`.
pub type SharedEdictChangeInfo = c_void;

// TODO: `IChangeInfoAccessor`.
vtable! {
	pub ChangeInfoAccessorVt {}
}

// TODO: `IAchievementMgr`.
vtable! {
	pub AchievementMgrVt {}
}

// TODO: `player_info_t`.
#[derive(Debug)]
#[repr(C)]
pub struct PlayerInfo {}

// TODO: `GamestatsData`.
#[derive(Debug)]
#[repr(C)]
pub struct GamestatsData {}

#[derive(Debug)]
#[repr(C)]
pub struct Bbox {
	pub mins: Vector,
	pub maxs: Vector,
}

// TODO: `IServer`.
vtable! {
	pub ServerVt {}
}

pub const INTERFACEVERSION_VENGINESERVER: &CStr = c"VEngineServer023";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FindMapResult {
	/// A direct match for the specified map name was found.
	Found,
	/// No match for the specified map name could be found.
	NotFound,
	/// A fuzzy match for the specified map name was found and `in_out_map_name` was updated to the full name.
	FuzzyMatch,
	/// A match for the specified map name was found,
	/// and the map name was updated to the canonical version of the name.
	/// 
	/// For example, `workshop/1234` would become `workshop/cp_qualified_name.ugc1234`.
	NonCanonical,
	/// No currently available match for the specified map name could be found,
	/// but it may be possible to load.
	/// 
	/// `find_map` is non-blocking,
	/// so it is possible that blocking resources in a
	/// [`ServerGameDllVt::prepare_level_resources`](super::ServerGameDllVt::prepare_level_resources)
	/// call may be able to pull a better match than is immediately available with `find_map`
	/// (e.g. blocking lookups of cloud maps).
	PossiblyAvailable,
}

vtable! {
	pub VEngineServerVt {
		pub fn change_level(s1: *const c_char, s2: *const c_char);
		pub fn is_map_valid(filename: *const c_char) -> c_int;
		pub fn is_dedicated_server() -> bool;
		pub fn is_in_edit_mode() -> c_int;
		pub fn precache_model(s: *const c_char, preload: bool) -> c_int;
		pub fn precache_sentence_file(s: *const c_char, preload: bool) -> c_int;
		pub fn precache_decal(name: *const c_char, preload: bool) -> c_int;
		pub fn precache_generic(s: *const c_char, preload: bool) -> c_int;
		pub fn is_model_precached(s: *const c_char) -> bool;
		pub fn is_decal_precached(s: *const c_char) -> bool;
		pub fn is_generic_precached(s: *const c_char) -> bool;
		pub fn get_cluster_for_origin(org: RefConst<Vector>);
		pub fn get_pvs_for_cluster(cluster: c_int, output_pvs_length: c_int, output_pvs: *mut c_uchar) -> c_int;
		pub fn check_origin_in_pvs(org: RefConst<Vector>, check_pvs: *const c_uchar, check_pvs_size: c_int) -> bool;
		pub fn check_box_in_pvs(
			mins: RefConst<Vector>, maxs: RefConst<Vector>,
			check_pvs: *const c_uchar, check_pvs_size: c_int,
		) -> bool;
		pub fn get_player_user_id(e: *const edict_t) -> c_int;
		pub fn get_player_network_id_string(e: *const edict_t) -> *const c_char;
		pub fn get_entity_count() -> c_int;
		pub fn index_of_edict(edict: *const edict_t) -> c_int;
		pub fn entity_of_ent_index(ent_index: c_int) -> *mut edict_t;
		pub fn get_player_net_info(player_index: c_int) -> VtObjectMut<NetChannelInfoVt>;
		pub fn create_edict(force_edict_index: c_int) -> *mut edict_t;
		pub fn remove_edict(e: *mut edict_t);
		pub fn alloc_ent_private_data(cb: c_long) -> *mut c_void;
		pub fn free_ent_private_data(entity: *mut c_void);
		pub fn save_alloc_memory(num: usize, size: usize) -> *mut c_void;
		pub fn save_free_memory(save_mem: *mut c_void);
		pub fn emit_ambient_sound(
			ent_index: c_int,
			pos: RefConst<Vector>, samp: *const c_char,
			vol: c_float, sound_level: SoundLevel,
			flags: c_int, pitch: c_int, delay: c_float,
		);
		pub fn fade_client_volume(
			edict: *const edict_t,
			fade_percent: c_float, fade_out_seconds: c_float, hold_time: c_float, fade_in_seconds: c_float,
		);
		pub fn sentence_group_pick(group_index: c_int, out_name: *mut c_char, name_buf_len: c_int) -> c_int;
		pub fn sentence_group_pick_sequential(
			group_index: c_int,
			out_name: *mut c_char, name_buf_len: c_int,
			sentence_index: c_int, reset: c_int,
		) -> c_int;
		pub fn sentence_index_from_name(sentence_name: *const c_char) -> c_int;
		pub fn sentence_name_from_index(sentence_index: c_int) -> *const c_char;
		pub fn sentence_group_index_from_name(group_name: *const c_char) -> c_int;
		pub fn sentence_group_name_from_index(group_index: c_int) -> *const c_char;
		pub fn sentence_length(sentence_index: c_int) -> c_float;
		pub fn server_command(str: *const c_char);
		pub fn server_execute();
		pub fn client_command(edict: *mut edict_t, fmt: *const c_char, ...);
		pub fn light_style(style: c_int, val: *const c_char);
		pub fn static_decal(
			origin_in_entity_space: RefConst<Vector>,
			decal_index: c_int, entity_index: c_int, model_index: c_int,
			low_priority: bool,
		);
		pub fn message_determine_multicast_recipients(
			use_pas: bool, origin: RefConst<Vector>, player_bits: RefMut<BitVec>,
		);
		pub fn entity_message_begin(ent_index: c_int, ent_class: *mut ServerClass, reliable: bool) -> *mut BfWrite;
		pub fn user_message_begin(filter: VtObjectMut<RecipientFilterVt>, msg_type: c_int) -> *mut BfWrite;
		pub fn message_end();
		pub fn client_printf(edict: *mut edict_t, msg: *const c_char);
		pub fn con_n_printf(pos: c_int, fmt: *const c_char, ...);
		pub fn con_nx_printf(info: *const ConNPrint, fmt: *const c_char, ...);
		pub fn set_view(client: *const edict_t, view_ent: *const edict_t);
		pub fn time() -> c_float;
		pub fn crosshair_angle(client: *const edict_t, pitch: c_float, yaw: c_float);
		pub fn get_game_dir(out_game_dir: *mut c_char, max_length: c_int);
		pub fn compare_file_time(filename1: *const c_char, filename2: *const c_char, out_compare: *mut c_int) -> c_int;
		pub fn lock_network_string_tables(lock: bool) -> bool;
		pub fn create_fake_client(net_name: *const c_char) -> *mut edict_t;
		pub fn get_client_convar_value(client_index: c_int, name: *const c_char) -> *const c_char;
		pub fn parse_file(data: *const c_char, out_token: *mut c_char, max_len: c_int) -> *const c_char;
		pub fn copy_file(source: *const c_char, destination: *const c_char) -> bool;
		pub fn reset_pvs(pvs: *mut u8, pvs_size: c_int);
		pub fn add_origin_to_pvs(origin: RefConst<Vector>);
		pub fn set_area_portal_state(portal_number: c_int, is_open: c_int);
		pub fn playback_temp_entity(
			filter: VtObjectMut<RecipientFilterVt>,
			delay: c_float, sender: *const c_void, st: *const SendTable, class_id: c_int,
		);
		pub fn check_headnode_visible(node_num: c_int, pvs: *const u8, vis_size: c_int) -> c_int;
		pub fn check_areas_connected(area1: c_int, area2: c_int) -> c_int;
		pub fn get_area(origin: RefConst<Vector>) -> c_int;
		pub fn get_area_bits(area: c_int, bits: *mut c_uchar, buf_len: c_int);
		pub fn get_area_portal_plane(view_origin: RefConst<Vector>, portal_key: c_int, out_plane: *mut VPlane) -> bool;
		pub fn load_game_state(map_name: *const c_char, create_players: bool) -> bool;
		pub fn load_adjacent_ents(old_level: *const c_char, landmark_name: *const c_char);
		pub fn clear_save_dir();
		pub fn get_map_entities_string() -> *const c_char;
		pub fn text_message_get(name: *const c_char) -> *mut ClientTextMessage;
		pub fn log_print(msg: *const c_char);
		pub fn build_entity_cluster_list(edict: *mut edict_t, out_pvs_info: *mut PvsInfo);
		pub fn solid_moved(
			solid_ent: *mut edict_t, solid_collide: VtObjectMut<CollideableVt>,
			prev_abs_origin: *const Vector,
			test_surrounding_bounds_only: bool,
		);
		pub fn trigger_moved(trigger_ent: *mut edict_t, test_surrounding_bounds_only: bool);
		pub fn create_spatial_partition(
			world_min: RefConst<Vector>, world_max: RefConst<Vector>,
		) -> VtObjectMut<SpatialPartitionVt>;
		pub fn destroy_spatial_partition(VtObjectMut<SpatialPartitionVt>);
		pub fn draw_map_to_scratch_pad(pad: VtObjectMut<ScratchPad3DVt>, flags: c_int);
		pub fn get_entity_transmit_bits_for_client(client_index: c_int) -> *const BitVec;
		pub fn is_paused() -> bool;
		pub fn force_exact_file(s: *const c_char);
		pub fn force_model_bounds(s: *const c_char, mins: RefConst<Vector>, maxs: RefConst<Vector>);
		pub fn clear_save_dir_after_client_load();
		pub fn set_fake_client_convar_value(entity: *mut edict_t, cvar: *const c_char, value: *const c_char);
		pub fn force_simple_material(s: *const c_char);
		pub fn is_in_commentary_mode() -> bool;
		pub fn set_area_portal_states(portal_numbers: *const c_int, is_open: *const c_int, n_portals: c_int);
		pub fn notify_edict_flags_change(edict: c_int);
		pub fn get_prev_check_transmit_info(player_edict: *mut edict_t) -> *const CheckTransmitInfo;
		pub fn get_shared_edict_change_info() -> *mut SharedEdictChangeInfo;
		pub fn allow_immediate_edict_reuse();
		pub fn is_internal_build() -> bool;
		pub fn get_change_accessor(edict: *const edict_t) -> VtObjectMut<ChangeInfoAccessorVt>;
		pub fn get_most_recently_load_file_name() -> *const c_char;
		pub fn get_save_file_name() -> *const c_char;
		pub fn multiplayer_end_game();
		pub fn change_team(team_name: *const c_char);
		pub fn clean_up_entity_cluster_list(pvs_info: *mut PvsInfo);
		pub fn set_achievement_mgr(achievement_mgr: VtObjectMut<AchievementMgrVt>);
		pub fn get_achievement_mgr() -> VtObjectMut<AchievementMgrVt>;
		pub fn get_app_id() -> c_int;
		pub fn is_low_violence() -> bool;
		pub fn start_query_cvar_value(player_entity: *mut edict_t, name: *const c_char);
		pub fn insert_server_command(str: *const c_char);
		pub fn get_player_info(ent_num: c_int, out_pinfo: *mut PlayerInfo) -> bool;
		pub fn is_client_fully_authenticated(edict: *mut edict_t) -> bool;
		pub fn set_dedicated_server_benchmark_mode(benchmark_mode: bool);
		pub fn set_gamestats_data(gamestats_data: *mut GamestatsData);
		pub fn get_gamestats_data() -> *mut GamestatsData;
		pub fn get_client_steam_id(player_edict: *mut edict_t) -> *const SteamId;
		pub fn get_game_server_steam_id() -> *const SteamId;
		pub fn client_command_key_values(edict: *mut edict_t, command: *mut KeyValues);
		pub fn get_client_steam_id_by_player_index(ent_num: c_int) -> *const SteamId;
		pub fn get_cluster_count() -> c_int;
		pub fn get_all_cluster_bounds(bbox_list: *mut Bbox, max_bbox: c_int) -> c_int;
		pub fn create_fake_client_ex(net_name: *const c_char, report_fake_client: bool) -> *mut edict_t;
		pub fn get_server_version() -> c_int;
		pub fn get_server_time() -> c_float;
		pub fn get_iserver() -> VtObjectMut<ServerVt>;
		pub fn is_player_name_locked(edict: *const edict_t) -> bool;
		pub fn can_player_change_name(edict: *const edict_t) -> bool;
		pub fn find_map(in_out_map_name: *mut c_char, map_name_max: c_int) -> FindMapResult;
		pub fn set_paused_forced(paused: bool, duration: c_float);
	}
}

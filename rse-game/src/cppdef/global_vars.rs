use ::core::{
	ffi::{
		c_float, c_int,
	},
	ptr::null_mut,
};

use super::string_t;

#[derive(Default, Debug)]
#[repr(C)]
pub struct GlobalVars {
	pub base: GlobalVarsBase,

	pub map_name: string_t,
	pub map_version: c_int,
	pub start_spot: string_t,
	pub load_type: MapLoadType,
	pub map_load_failed: bool,

	pub deathmatch: bool,
	pub coop: bool,
	pub teamplay: bool,

	pub max_entities: c_int,

	pub server_count: c_int,
}

impl GlobalVars {
	pub const fn new(is_client: bool) -> Self {
		Self {
			base: GlobalVarsBase::new(is_client),
			map_name: string_t::NULL,
			map_version: 0,
			start_spot: string_t::NULL,
			load_type: MapLoadType::NewGame,
			map_load_failed: false,
			deathmatch: false,
			coop: false,
			teamplay: false,
			max_entities: 0,
			server_count: 0,
		}
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum MapLoadType {
	#[default]
	NewGame = 0,
	LoadGame,
	Transition,
	Background,
}

#[derive(Default, Debug)]
pub struct GlobalVarsBase {
	/// Absolute time.
	// TODO: See what `Plat_FloatTime` does to suggest using that.
	pub real_time: c_float,
	/// Absolute frame counter.
	pub frame_count: c_int,
	/// Non-paused time it took to render a frame.
	pub absolute_frame_time: c_float,
	/// Current time.
	/// 
	/// On the client, this (along with tick count) takes a different meaning based on what piece of code you're in:
	/// - While receiving network packets (like in PreDataUpdate/PostDataUpdate and proxies),
	///   this is set to the *server tick count* for that packet.
	///   There is no interval between the server ticks.
	///   The value is equal to `server_current_tick * tick_interval`.
	/// - While rendering, this is the exact client clock 
	///   The value is equal to `client_current_tick * tick_interval + interpolation_amount`.
	/// - During prediction, this is based on the client's current tick:
	///   The value is equal to `client_current_tick * tick_interval`.
	pub current_time: c_float,
	/// Time spent on last server or client frame.
	/// 
	/// This value is not related to think intervals.
	pub frame_time: c_float,
	/// Current `maxplayers` setting.
	pub max_clients: c_int,
	/// Number of simulation ticks.
	pub tick_count: c_int,
	/// Interval at which simulation ticks happen.
	pub interval_per_tick: c_float,
	/// **Client-only:**
	/// Interpolation amount based on fraction of next tick which has elapsed.
	pub interpolation_amount: c_float,
	pub sim_ticks_this_frame: c_int,
	pub network_protocol: c_int,
	/// Current data for the save-restore system.
	pub save_data: *mut SaveRestoreData,
	/// `true` if running on the client.
	pub is_client: bool,
	/// Value which is used in the [`network_base`](Self::network_base) calculation.
	/// 
	/// The default value is `100`.
	pub timestamp_networking_base: c_int,
	/// Value which is used in the [`network_base`](Self::network_base) calculation.
	/// 
	/// This value prevents all of the entities from forcing a new packed entity to be created on the same tick;
	/// that is, it prevents them from getting lockstepped on this.
	/// 
	/// The default value is `32`.
	pub timestamp_randomize_window: c_int
}

impl GlobalVarsBase {
	pub const fn new(is_client: bool) -> Self {
		Self {
			real_time: 0.0,
			frame_count: 0,
			absolute_frame_time: 0.0,
			current_time: 0.0,
			frame_time: 0.0,
			max_clients: 0,
			tick_count: 0,
			interval_per_tick: 0.0,
			interpolation_amount: 0.0,
			sim_ticks_this_frame: 0,
			network_protocol: 0,
			save_data: null_mut(),
			is_client,
			timestamp_networking_base: 100,
			timestamp_randomize_window: 32,
		}
	}

	// TODO: "For encoding `m_flSimulationTime` and `m_flAnimTime`" - what does this mean?
	pub const fn network_base(&self, tick: c_int, entity: c_int) -> c_int {
		let entity_mod = entity % self.timestamp_randomize_window;
		self.timestamp_networking_base * ((tick - entity_mod) / self.timestamp_networking_base)
	}
}

// TODO: `CSaveRestoreData`.
pub type SaveRestoreData = ::core::ffi::c_void;

use ::core::ffi::{
	CStr, c_char, c_float, c_int, c_uint,
};
use ::rse_cpp::{
	class,
	RefMut,
};
use ::rse_math::{
	Vector, QAngle,
};
use ::rse_utl::cppdef::{
	CUtlVector,
	FileNameHandle_t,
};

use crate::cppdef::{
	IRecipientFilter,
	soundlevel_t,
};

class! {
	pub IEngineSound {
		pub virtual fn precache_sound(sample: *const c_char, preload: bool, is_ui_sound: bool) -> bool;
		pub virtual fn is_sound_precached(sample: *const c_char) -> bool;
		pub virtual fn prefetch_sound(sample: *const c_char);

		pub virtual fn get_sound_duration(sample: *const c_char) -> c_float;

		pub virtual fn emit_sound_with_attenuation(
			filter: RefMut<IRecipientFilter>,
			ent_index: c_int, channel: c_int,
			sample: *const c_char,
			volume: c_float, attenuation: c_float,
			flags: c_int, pitch: c_int, special_dsp: c_int,
			origin: *const Vector, direction: *const Vector,
			origins: *mut CUtlVector<Vector>, update_positions: bool, sound_time: c_float,
			speaker_entity: c_int,
		);
		pub virtual fn emit_sound_with_sndlvl(
			filter: RefMut<IRecipientFilter>,
			ent_index: c_int, channel: c_int,
			sample: *const c_char,
			volume: c_float, sound_level: soundlevel_t,
			flags: c_int, pitch: c_int, special_dsp: c_int,
			origin: *const Vector, direction: *const Vector,
			origins: *mut CUtlVector<Vector>, update_positions: bool, sound_time: c_float,
			speaker_entity: c_int,
		);
		pub virtual fn emit_sentence_by_index(
			filter: RefMut<IRecipientFilter>,
			ent_index: c_int, channel: c_int,
			sentence_index: c_int,
			volume: c_float, sound_level: soundlevel_t,
			flags: c_int, pitch: c_int, special_dsp: c_int,
			origin: *const Vector, direction: *const Vector,
			origins: *mut CUtlVector<Vector>, update_positions: bool, sound_time: c_float,
			speaker_entity: c_int,
		);

		pub virtual fn stop_sound(ent_index: c_int, channel: c_int, sample: *const c_char);

		pub virtual fn stop_all_sounds(clear_buffers: bool);

		pub virtual fn set_room_type(filter: RefMut<IRecipientFilter>, room_type: c_int);

		pub virtual fn set_player_dsp(filter: RefMut<IRecipientFilter>, dsp_type: c_int, fast_reset: bool);

		pub virtual fn emit_ambient_sound(
			sample: *const c_char, volume: c_float, pitch: c_int, flags: c_int, sound_time: c_float,
		);

		pub virtual fn get_dist_gain_from_sound_level(sound_level: soundlevel_t, dist: c_float) -> c_float;

		pub virtual fn get_guid_for_last_sound_emitted() -> c_int;
		pub virtual fn is_sound_still_playing(guid: c_int) -> bool;
		pub virtual fn stop_sound_by_guid(guid: c_int);
		pub virtual fn set_volume_by_guid(guid: c_int, vol: c_float);

		pub virtual fn get_all_active_sounds(snd_list: RefMut<CUtlVector<SndInfo_t>>);

		pub virtual fn precache_sentence_group(group_name: *const c_char);
		pub virtual fn notify_begin_movie_playback();
		pub virtual fn notify_end_movie_playback();

		pub virtual fn create_output_stream(
			sample_rate: c_uint, channels: c_uint, bits: c_uint,
		) -> *mut IAudioOutputStream;
		pub virtual fn destroy_output_stream(stream: *mut IAudioOutputStream);

		pub virtual fn manual_update(listener_state: *const AudioState_t);
		pub virtual fn extra_update();
	}
}

pub const SOUND_FROM_UI_PANEL: c_int = -2;
pub const SOUND_FROM_LOCAL_PLAYER: c_int = -1;
pub const SOUND_FROM_WORLD: c_int = 0;

class! {
	pub IAudioOutputStream {
		pub virtual ~destructors();
		pub virtual fn write_audio_data(data: *const i16, sample_count: c_uint, channels: c_uint);
		pub virtual fn set_volume(volume: c_float);
		pub virtual fn queued_sample_count() -> u32;
		pub virtual fn max_write_sample_count() -> u32;
		pub virtual fn latency_samples_count() -> u32;
		pub virtual fn pause();
		pub virtual fn resume();
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct SndInfo_t {
	pub guid: c_int,
	pub file_name_handle: FileNameHandle_t,
	pub sound_source: c_int,
	pub channel: c_int,
	pub speaker_entity: c_int,
	pub volume: c_float,
	pub last_spatialized_volume: c_float,
	pub radius: c_float,
	pub pitch: c_int,
	pub origin: *mut Vector,
	pub direction: *mut Vector,
	pub update_positions: bool,
	pub is_sentence: bool,
	pub dry_mix: bool,
	pub speaker: bool,
	pub special_dsp: bool,
	pub from_server: bool,
}

pub const IENGINESOUND_CLIENT_INTERFACE_VERSION: &CStr = c"IEngineSoundClient003";
pub const IENGINESOUND_SERVER_INTERFACE_VERSION: &CStr = c"IEngineSoundServer003";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct AudioState_t {
	pub origin: Vector,
	pub angles: QAngle,
	pub is_underwater: bool,
}


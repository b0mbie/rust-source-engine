use ::core::ffi::{
	c_int, c_float, c_void,
};
use ::rse_cpp::{
	VtObjectMut, vtable,
};

use super::{
	Sentence,
	AudioDeviceVt,
	channel_t,
};

vtable! {
	pub AudioSourceVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		// TODO: Nullable?
		pub fn create_mixer() -> Option<VtObjectMut<AudioMixerVt>>;
		pub fn get_output_data(
			data: *mut *mut c_void,
			sample_position: c_int, sample_count: c_int,
			forward: bool,
		) -> c_int;
		pub fn sample_rate() -> c_int;
		pub fn sample_size() -> c_int;
		pub fn sample_count() -> c_int;
		pub fn true_sample_size() -> c_float;
		pub fn is_looped() -> bool;
		pub fn is_streaming() -> bool;
		pub fn get_running_length() -> c_float;
		pub fn get_num_channels() -> c_int;
		pub fn get_sentence() -> *mut Sentence;
	}
}

vtable! {
	pub AudioMixerVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn mix_data_to_device(
			device: VtObjectMut<AudioDeviceVt>,
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		) -> bool;
		pub fn increment_samples(
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		);
		pub fn skip_samples(
			device: VtObjectMut<AudioDeviceVt>,
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		) -> bool;
		pub fn get_source() -> VtObjectMut<AudioSourceVt>;
		pub fn get_sample_position() -> c_int;
		pub fn get_scrub_position() -> c_int;
		pub fn set_sample_position(position: c_int, scrubbing: bool) -> bool;
		pub fn set_loop_position(position: c_int);
		pub fn get_start_position() -> c_int;
		pub fn get_active() -> bool;
		pub fn set_active(active: bool);
		pub fn set_model_index(index: c_int);
		pub fn get_model_index() -> c_int;
		pub fn set_direction(forward: bool);
		pub fn get_direction() -> bool;
		pub fn set_auto_delete(autodelete: bool);
		pub fn get_auto_delete() -> bool;
		pub fn set_volume(volume: c_float);
		pub fn get_channel() -> *mut channel_t;
	}
}

use ::core::ffi::{
	c_int, c_float, c_void,
};
use ::rse_cpp::class;

use super::{
	CSentence,
	IAudioDevice,
	channel_t,
};

class! {
	pub CAudioSource {
		pub virtual ~destructors();
		// TODO: Nullable?
		pub virtual fn create_mixer() -> *mut CAudioMixer;
		pub virtual fn get_output_data(
			data: *mut *mut c_void,
			sample_position: c_int, sample_count: c_int,
			forward: bool,
		) -> c_int;
		pub virtual fn sample_rate() -> c_int;
		pub virtual fn sample_size() -> c_int;
		pub virtual fn sample_count() -> c_int;
		pub virtual fn true_sample_size() -> c_float;
		pub virtual fn is_looped() -> bool;
		pub virtual fn is_streaming() -> bool;
		pub virtual fn get_running_length() -> c_float;
		pub virtual fn get_num_channels() -> c_int;
		pub virtual fn get_sentence() -> *mut CSentence;
	}
}

class! {
	pub CAudioMixer {
		pub virtual ~destructors();
		pub virtual fn mix_data_to_device(
			device: *mut IAudioDevice,
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		) -> bool;
		pub virtual fn increment_samples(
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		);
		pub virtual fn skip_samples(
			device: *mut IAudioDevice,
			channel: *mut channel_t,
			start_sample: c_int, sample_count: c_int,
			output_rate: c_int,
			forward: bool,
		) -> bool;
		pub virtual fn get_source() -> *mut CAudioSource;
		pub virtual fn get_sample_position() -> c_int;
		pub virtual fn get_scrub_position() -> c_int;
		pub virtual fn set_sample_position(position: c_int, scrubbing: bool) -> bool;
		pub virtual fn set_loop_position(position: c_int);
		pub virtual fn get_start_position() -> c_int;
		pub virtual fn get_active() -> bool;
		pub virtual fn set_active(active: bool);
		pub virtual fn set_model_index(index: c_int);
		pub virtual fn get_model_index() -> c_int;
		pub virtual fn set_direction(forward: bool);
		pub virtual fn get_direction() -> bool;
		pub virtual fn set_auto_delete(autodelete: bool);
		pub virtual fn get_auto_delete() -> bool;
		pub virtual fn set_volume(volume: c_float);
		pub virtual fn get_channel() -> *mut channel_t;
	}
}

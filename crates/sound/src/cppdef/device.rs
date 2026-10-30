use ::core::ffi::{
	c_char, c_int, c_float, c_uint, c_short,
};

use super::CAudioMixer;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct channel_t {
	pub left_vol: c_int,
	pub right_vol: c_int,
	pub pitch: c_float,
}

pub type FixedInt = c_uint;

::rse_cpp::class! {
	pub IAudioDevice {
		pub virtual ~destructors();
		pub virtual fn init() -> bool;
		pub virtual fn shutdown();
		pub virtual fn device_name() -> *const c_char;
		pub virtual fn device_channels() -> c_int;
		pub virtual fn device_sample_bits() -> c_int;
		pub virtual fn device_sample_bytes() -> c_int;
		pub virtual fn device_sample_rate() -> c_int;
		pub virtual fn device_sample_count() -> c_int;
		pub virtual fn mix_begin();

		pub virtual fn mix_8_mono(
			channel: *mut channel_t,
			data: *mut c_char,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub virtual fn mix_8_stereo(
			channel: *mut channel_t,
			data: *mut c_char,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub virtual fn mix_16_mono(
			channel: *mut channel_t,
			data: *mut c_short,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub virtual fn mix_16_stereo(
			channel: *mut channel_t,
			data: *mut c_short,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);

		pub virtual fn paint_buffer_sample_count() -> c_int;
		pub virtual fn add_source(source: *mut CAudioMixer);
		pub virtual fn stop_sounds();
		pub virtual fn update();
		pub virtual fn flush();

		pub virtual fn find_source_index(source: *mut CAudioMixer);
	}
}

use ::core::ffi::{
	c_char, c_int, c_float, c_uint, c_short,
};
use ::rse_cpp::VtObjectPtr;

use super::AudioMixerVt;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct channel_t {
	pub left_vol: c_int,
	pub right_vol: c_int,
	pub pitch: c_float,
}

pub type FixedInt = c_uint;

::rse_cpp::vtable! {
	pub AudioDeviceVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn init() -> bool;
		pub fn shutdown();
		pub fn device_name() -> *const c_char;
		pub fn device_channels() -> c_int;
		pub fn device_sample_bits() -> c_int;
		pub fn device_sample_bytes() -> c_int;
		pub fn device_sample_rate() -> c_int;
		pub fn device_sample_count() -> c_int;
		pub fn mix_begin();

		pub fn mix_8_mono(
			channel: *mut channel_t,
			data: *mut c_char,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub fn mix_8_stereo(
			channel: *mut channel_t,
			data: *mut c_char,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub fn mix_16_mono(
			channel: *mut channel_t,
			data: *mut c_short,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);
		pub fn mix_16_stereo(
			channel: *mut channel_t,
			data: *mut c_short,
			output_offset: c_int, input_offset: c_int,
			rate_scale_fix: FixedInt,
			out_count: c_int,
			time_compress: c_int,
			forward: bool,
		);

		pub fn paint_buffer_sample_count() -> c_int;
		pub fn add_source(source: VtObjectPtr<AudioMixerVt>);
		pub fn stop_sounds();
		pub fn update();
		pub fn flush();

		pub fn find_source_index(source: VtObjectPtr<AudioMixerVt>);
	}
}

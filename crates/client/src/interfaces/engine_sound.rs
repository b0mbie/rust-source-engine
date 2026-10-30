use ::core::{
	ffi::{
		CStr, c_float, c_uint,
	},
	marker::PhantomData,
	num::NonZero,
	ops::{
		Deref, DerefMut,
	},
};
use ::rse_cpp::{
	util::clamp_len_to_c_uint,
	transparent_wrapper,
	virtual_call,
};
use ::rse_game_interfaces::{
	cppdef::{
		IEngineSound, IENGINESOUND_CLIENT_INTERFACE_VERSION,
		IAudioOutputStream,
	},
	EngineSound,
	InterfaceOfFactory,
};
use ::rse_interface::{
	Interface,
	ptr_interface,
};

ptr_interface! {
	pub struct OEngineSoundClient(EngineSoundClient);
	Pointee = IEngineSound;
}
impl Interface for OEngineSoundClient {
	const IDENTIFIER: &CStr = IENGINESOUND_CLIENT_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OEngineSoundClient {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}

impl EngineSoundClient {
	pub fn create_output_stream_raw(
		&mut self,
		sample_rate: c_uint, channels: c_uint, bits: c_uint,
	) -> *mut IAudioOutputStream {
		unsafe { virtual_call!(mut self.as_mut_inner() => create_output_stream(
			sample_rate, channels, bits,
		)) }
	}

	pub unsafe fn destroy_output_stream_raw(&mut self, stream: *mut IAudioOutputStream) {
		unsafe { virtual_call!(mut self.as_mut_inner() => destroy_output_stream(stream)) }
	}
}

impl Deref for EngineSoundClient {
	type Target = EngineSound;
	fn deref(&self) -> &Self::Target {
		unsafe { EngineSound::from_ref(self.as_inner()) }
	}
}
impl DerefMut for EngineSoundClient {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { EngineSound::from_mut(self.as_mut_inner()) }
	}
}

#[derive(Clone, Copy)]
pub struct AudioData<'a> {
	ptr: *const i16,
	sample_count: c_uint,
	channels: NonZero<c_uint>,
	_slice: PhantomData<&'a [i16]>,
}

impl<'a> AudioData<'a> {
	pub const fn new(data: &'a [i16], channels: NonZero<c_uint>) -> Option<Self> {
		let rem = data.len() % (channels.get() as usize);
		if rem == 0 {
			unsafe { Some(Self::new_unchecked(data, channels)) }
		} else {
			None
		}
	}

	pub const unsafe fn new_unchecked(data: &'a [i16], channels: NonZero<c_uint>) -> Self {
		Self {
			ptr: data.as_ptr(),
			sample_count: clamp_len_to_c_uint(data.len() / (channels.get() as usize)),
			channels,
			_slice: PhantomData,
		}
	}
}

impl AudioOutputStream {
	pub fn write_audio_data(&mut self, data: AudioData<'_>) {
		unsafe { virtual_call!(mut self.as_mut_inner() => write_audio_data(
			data.ptr, data.sample_count, data.channels.get(),
		)) }
	}

	pub fn set_volume(&mut self, volume: c_float) {
		unsafe { virtual_call!(mut self.as_mut_inner() => set_volume(volume)) }
	}

	pub fn queued_sample_count(&self) -> u32 {
		unsafe { virtual_call!(self.as_inner() => queued_sample_count()) }
	}

	pub fn max_write_sample_count(&self) -> u32 {
		unsafe { virtual_call!(self.as_inner() => max_write_sample_count()) }
	}

	pub fn latency_samples_count(&self) -> u32 {
		unsafe { virtual_call!(self.as_inner() => latency_samples_count()) }
	}

	pub fn pause(&mut self) {
		unsafe { virtual_call!(mut self.as_mut_inner() => pause()) }
	}

	pub fn resume(&mut self) {
		unsafe { virtual_call!(mut self.as_mut_inner() => resume()) }
	}
}
transparent_wrapper! {
	pub struct AudioOutputStream for IAudioOutputStream as "IAudioOutputStream";
}

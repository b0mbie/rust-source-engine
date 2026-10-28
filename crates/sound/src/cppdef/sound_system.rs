use ::core::ffi::{
	CStr, c_char, c_float, c_ushort,
};
use ::rse_interface::cppdef::app_system::IAppSystem;

use super::{
	CAudioMixer, CAudioSource,
};

#[allow(non_camel_case_types)]
pub type AudioSourceHandle_t = c_ushort;
pub const AUDIOSOURCEHANDLE_INVALID: AudioSourceHandle_t = !0;

::rse_cpp::class! {
	pub ISoundSystem: pub virtual IAppSystem {
		pub virtual fn update(time: c_float);
		pub virtual fn flush();
		pub virtual fn find_or_add_sound(filename: *const c_char) -> *mut CAudioSource;
		pub virtual fn load_sound(wavfile: *const c_char) -> *mut CAudioSource;
		pub virtual fn play_sound(source: *mut CAudioSource, volume: c_float, mixer: *mut *mut CAudioMixer);
		pub virtual fn is_sound_playing(mixer: *mut CAudioMixer) -> bool;
		pub virtual fn find_mixer(source: *mut CAudioSource) -> *mut CAudioMixer;
		pub virtual fn stop_all();
		pub virtual fn stop_sound(mixer: *mut CAudioMixer);
	}
}

pub const SOUNDSYSTEM_INTERFACE_VERSION: &CStr = c"SoundSystem001";

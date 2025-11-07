use ::core::ffi::{
	c_char, c_float, c_ushort,
};
use ::rse_utl::cppdef::UtlVector;

#[cfg(feature = "phoneme-editor")]
use ::core::ffi::{
	c_int, c_uint,
};

#[repr(C)]
pub struct Sentence {
	#[cfg(feature = "phoneme-editor")]
	pub text: *const c_char,
	#[cfg(feature = "phoneme-editor")]
	pub words: UtlVector<*mut WordTag>,
	pub run_time_phonemes: UtlVector<*mut BasePhonemeTag>,
	#[cfg(feature = "phoneme-editor")]
	pub reset_word_base: c_int,
	pub emphasis_samples: UtlVector<EmphasisSample>,
	#[cfg(feature = "phoneme-editor")]
	pub check_sum: c_uint,
	// FIXME: bool : 8
	pub is_valid: u8,
	// FIXME: bool : 8
	pub store_check_sum: u8,
	// FIXME: bool : 8
	pub should_voice_duck: u8,
	// FIXME: bool : 8
	pub is_cached: u8,
}

#[repr(C)]
pub struct WordTag {
	pub start_time: c_float,
	pub end_time: c_float,
	pub phonemes: UtlVector<*mut PhonemeTag>,
	#[cfg(feature = "phoneme-editor")]
	pub selected: bool,
	#[cfg(feature = "phoneme-editor")]
	pub start_byte: c_uint,
	#[cfg(feature = "phoneme-editor")]
	pub end_byte: c_uint,
	pub word: *mut c_char,
}

#[repr(C)]
pub struct PhonemeTag {
	pub base: BasePhonemeTag,
	#[cfg(feature = "phoneme-editor")]
	pub selected: bool,
	#[cfg(feature = "phoneme-editor")]
	pub start_byte: c_uint,
	#[cfg(feature = "phoneme-editor")]
	pub end_byte: c_uint,
	pub phoneme: *mut c_char,
}

#[repr(C)]
pub struct BasePhonemeTag {
	pub start_time: c_float,
	pub end_time: c_float,
	pub phoneme_code: c_ushort,
}

#[repr(C)]
pub struct EmphasisSample {

}

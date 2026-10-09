use ::core::ffi::{
	c_char, c_float, c_ushort,
};
use ::rse_cpp::class;
use ::rse_utl::cppdef::CUtlVector;

#[cfg(feature = "phoneme-editor")]
use ::core::ffi::{
	c_int, c_uint,
};

class! {
	pub CSentence {
		#[cfg(feature = "phoneme-editor")]
		pub text: *const c_char;
		#[cfg(feature = "phoneme-editor")]
		pub words: CUtlVector<*mut CWordTag>;
		pub run_time_phonemes: CUtlVector<*mut CBasePhonemeTag>;
		#[cfg(feature = "phoneme-editor")]
		pub reset_word_base: c_int;
		pub emphasis_samples: CUtlVector<CEmphasisSample>;
		#[cfg(feature = "phoneme-editor")]
		pub check_sum: c_uint;
		// FIXME: bool : 8
		pub is_valid: u8;
		// FIXME: bool : 8
		pub store_check_sum: u8;
		// FIXME: bool : 8
		pub should_voice_duck: u8;
		// FIXME: bool : 8
		pub is_cached: u8;
	}
}

class! {
	pub CWordTag {
		pub start_time: c_float;
		pub end_time: c_float;
		pub phonemes: CUtlVector<*mut CPhonemeTag>;
		#[cfg(feature = "phoneme-editor")]
		pub selected: bool;
		#[cfg(feature = "phoneme-editor")]
		pub start_byte: c_uint;
		#[cfg(feature = "phoneme-editor")]
		pub end_byte: c_uint;
		pub word: *mut c_char;
	}
}

class! {
	pub CPhonemeTag: pub CBasePhonemeTag {
		#[cfg(feature = "phoneme-editor")]
		pub selected: bool;
		#[cfg(feature = "phoneme-editor")]
		pub start_byte: c_uint;
		#[cfg(feature = "phoneme-editor")]
		pub end_byte: c_uint;
		pub phoneme: *mut c_char;
	}
}

class! {
	pub CBasePhonemeTag {
		pub start_time: c_float;
		pub end_time: c_float;
		pub phoneme_code: c_ushort;
	}
}

#[repr(C)]
pub struct CEmphasisSample {
	pub time: c_float,
	pub value: c_float,
	#[cfg(feature = "phoneme-editor")]
	pub selected: bool,
}

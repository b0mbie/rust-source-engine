use ::core::ffi::{
	c_char, c_float, c_int, c_uchar, c_void,
};
use ::rse_cpp::wide_c_str::wchar_t;

::rse_cpp::class! {
	pub KeyValues {
		pub key_name: c_int;
		pub s_value: *mut c_char;
		pub ws_value: *mut wchar_t;
		pub value: KeyValuesValue;
		pub data_type: c_char;
		pub has_escape_sequences: bool;
		pub evaluate_conditionals: bool;
		pub unused: [c_char; 1];
		pub peer: *mut Self;
		pub sub: *mut Self;
		pub chain: *mut Self;
	}
}

#[repr(C)]
pub union KeyValuesValue {
	pub i_value: c_int,
	pub fl_value: c_float,
	pub p_value: *mut c_void,
	pub color: [c_uchar; 4],
}

#[repr(C)]
pub enum types_t {
	None = 0,
	String,
	Int,
	Float,
	Ptr,
	WString,
	Color,
	UInt64,
	NumTypes,
}

pub const INVALID_KEY_SYMBOL: c_int = -1;
pub type HKeySymbol = c_int;

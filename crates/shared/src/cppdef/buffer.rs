use ::core::ffi::{
	c_char, c_int, c_uchar,
};
use ::rse_cpp::class;

class! {
	pub bf_read {
		pub data: *const c_uchar;
		pub data_bytes: c_int;
		pub data_bits: c_int;
		pub current_bit: c_int;
		pub overflow: bool;
		pub assert_on_overflow: bool;
		pub debug_name: *const c_char;
	}
}

class! {
	pub bf_write {
		pub data: *mut u32;
		pub data_bytes: c_int;
		pub data_bits: c_int;
		pub current_bit: c_int;
		pub overflow: bool;
		pub assert_on_overflow: bool;
		pub debug_name: *const c_char;
	}
}

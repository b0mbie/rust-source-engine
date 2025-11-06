use ::core::ffi::{
	c_char, c_int, c_uchar,
};

#[derive(Debug)]
#[repr(C)]
pub struct BfRead {
	pub data: *const c_uchar,
	pub data_bytes: c_int,
	pub data_bits: c_int,
	pub current_bit: c_int,
	pub overflow: bool,
	pub assert_on_overflow: bool,
	pub debug_name: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct BfWrite {
	pub data: *mut u32,
	pub data_bytes: c_int,
	pub data_bits: c_int,
	pub current_bit: c_int,
	pub overflow: bool,
	pub assert_on_overflow: bool,
	pub debug_name: *const c_char,
}

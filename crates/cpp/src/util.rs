use ::core::ffi::{
	c_int, c_uint,
};

pub const fn clamp_len_to_c_int(x: usize) -> c_int {
	if x > (c_int::MAX as usize) {
		c_int::MAX
	} else {
		x as c_int
	}
}

pub const fn clamp_len_to_c_uint(x: usize) -> c_uint {
	if x > (c_uint::MAX as usize) {c_uint::MAX} else {x as _}
}

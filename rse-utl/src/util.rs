use ::core::ffi::c_int;

pub const fn clamp_len_to_c_int(x: usize) -> c_int {
	if x > (c_int::MAX as usize) {
		c_int::MAX
	} else {
		x as c_int
	}
}

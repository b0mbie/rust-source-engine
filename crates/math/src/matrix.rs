use ::core::ffi::c_float;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct matrix3x4_t {
	pub values: [[c_float; 3]; 4],
}

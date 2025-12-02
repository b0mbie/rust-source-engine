use ::core::ffi::c_float;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Matrix3x4 {
	pub values: [[c_float; 3]; 4],
}

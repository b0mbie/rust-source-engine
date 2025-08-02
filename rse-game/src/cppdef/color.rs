use ::core::ffi::c_uchar;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color {
	// These correctly correspond to `[c_uchar; 4]`.
	pub r: c_uchar,
	pub g: c_uchar,
	pub b: c_uchar,
	pub a: c_uchar,
}

use ::core::ffi::c_float;

use super::Vector;

// TODO: In code, this is named `cplane_t`, but is called `plane_t` in the comment above it. Figure out why.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Plane {
	pub normal: Vector,
	pub distance: c_float,
	pub ty: u8,
	pub sign_bits: u8,
	pub _padding: [u8; 2],
}

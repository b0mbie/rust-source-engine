use ::core::ffi::c_float;

/// Source Engine 2D vector type.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector2D {
	pub x: vec_t,
	pub y: vec_t,
}

/// Source Engine 3D vector type.
// Cross-referenced with `vector.h` in `mathlib`.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector {
	pub x: vec_t,
	pub y: vec_t,
	pub z: vec_t,
}

impl Vector {
	/// Returns a new 3D vector from its components.
	pub const fn new(x: vec_t, y: vec_t, z: vec_t) -> Self {
		Self {
			x, y, z,
		}
	}
}

/// Vector component type.
// From `basetypes.h` in `tier0`.
#[allow(non_camel_case_types)]
pub type vec_t = c_float;

#![no_std]

use ::core::{
	ffi::c_float,
	ops::{
		Deref, DerefMut,
	},
};

/// Vector component type.
// From `basetypes.h` in `tier0`.
#[allow(non_camel_case_types)]
type vec_t = c_float;

/// Source Engine 2D vector type.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2D {
	pub x: vec_t,
	pub y: vec_t,
}

/// Source Engine 3D vector type.
// Cross-referenced with `vector.h` in `mathlib`.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
	pub x: vec_t,
	pub y: vec_t,
	pub z: vec_t,
}

impl Vector {
	/// Create a new 3D vector from its components.
	pub const fn new(x: vec_t, y: vec_t, z: vec_t) -> Self {
		Self {
			x, y, z,
		}
	}
}

/// Aligned version of [`Vector`] for SSE optimizations.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(align(16))]
pub struct VectorAligned(pub Vector);
impl Deref for VectorAligned {
	type Target = Vector;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for VectorAligned {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl From<Vector> for VectorAligned {
	fn from(value: Vector) -> Self {
		Self(value)
	}
}
impl From<VectorAligned> for Vector {
	fn from(value: VectorAligned) -> Self {
		value.0
	}
}

/// Vector that represents three-dimensional extrinsic Tait-Bryan rotations following the right-hand rule,
/// offset from the cardinal Z axis.
pub type QAngle = Vector;

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

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Matrix3x4 {
	pub values: [[c_float; 3]; 4],
}

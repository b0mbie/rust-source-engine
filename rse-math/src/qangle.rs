use super::vec_t;

/// Vector that represents three-dimensional extrinsic Tait-Bryan rotations following the right-hand rule,
/// offset from the cardinal Z axis.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct QAngle {
	pub x: vec_t,
	pub y: vec_t,
	pub z: vec_t,
}

impl QAngle {
	/// Returns a new 3D Tait-Bryan angle vector from its components.
	pub const fn new(x: vec_t, y: vec_t, z: vec_t) -> Self {
		Self {
			x, y, z,
		}
	}
}

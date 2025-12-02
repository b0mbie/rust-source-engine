use super::{
	Vector, vec_t,
};

#[repr(C)]
pub struct VPlane {
	pub normal: Vector,
	pub dist: vec_t,
}

impl VPlane {
	pub const EPSILON: vec_t = 0.01;
}

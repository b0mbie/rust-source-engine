use ::core::ops::{
	Deref, DerefMut,
};

use super::Vector;

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

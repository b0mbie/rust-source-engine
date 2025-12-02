use ::core::ops::Deref;
use ::rse_utl::{
	cppdef::UtlVector,
	memory::tier0::Tier0Memory,
	Vector,
};

use crate::cppdef::{
	CspVertList, CspVert,
};

::rse_cpp::transparent_wrapper! {
	pub struct VertList for CspVertList as "CspVertList";
}

impl VertList {
	pub const fn vec(&self) -> &Vector<CspVert> {
		unsafe { Vector::from_ptr(&self.0.verts as *const _ as *const UtlVector<CspVert, Tier0Memory<CspVert>>) }
	}
}

impl Deref for VertList {
	type Target = [CspVert];
	fn deref(&self) -> &Self::Target {
		self.vec().as_slice()
	}
}

use ::core::ops::Deref;
use ::rse_utl::{
	cppdef::CUtlVector,
	memory::tier0::Tier0Memory,
	Vector,
};

use crate::cppdef::{
	CSPVertList, CSPVert,
};

::rse_cpp::transparent_wrapper! {
	pub struct VertList for CSPVertList as "CspVertList";
}

impl VertList {
	pub const fn vec(&self) -> &Vector<CSPVert> {
		unsafe { Vector::from_ptr(&self.0.verts as *const _ as *const CUtlVector<CSPVert, Tier0Memory<CSPVert>>) }
	}
}

impl Deref for VertList {
	type Target = [CSPVert];
	fn deref(&self) -> &Self::Target {
		self.vec().as_slice()
	}
}

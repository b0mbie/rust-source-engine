use ::core::ffi::c_ushort;

use super::CUtlRBTree;

::rse_cpp::class! {
	#[derive(Debug)]
	pub CUtlMap<K, T, I = c_ushort> {
		pub tree: CUtlRBTree<Node_t<K, T>, I>;
	}
}

#[derive(Debug)]
#[repr(C)]
pub struct Node_t<K, T> {
	pub key: K,
	pub elem: T,
}

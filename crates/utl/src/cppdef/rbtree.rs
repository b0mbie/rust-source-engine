use ::core::ffi::c_ushort;
use ::rse_cpp::class;

use super::CUtlMemory;

class! {
	#[derive(Debug)]
	pub CUtlRBTree<
		T, I = c_ushort,
		/* `L` omitted as it is supposed to be a `LessFunc_t<T>` */
		M = CUtlMemory<UtlRBTreeNode_t<T, I>>,
	> {
		pub less_func: super::LessFunc_t<T>;
		pub elements: M;
		pub root: I;
		pub num_elements: I;
		pub first_free: I;
		pub p_elements: *mut UtlRBTreeNode_t<T, I>;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum NodeColor_t {
	Red = 0,
	Black,
}

#[derive(Debug)]
#[repr(C)]
pub struct UtlRBTreeLinks_t<I> {
	pub left: I,
	pub right: I,
	pub parent: I,
	pub tag: I,
}

#[derive(Debug)]
#[repr(C)]
pub struct UtlRBTreeNode_t<T, I> {
	pub links: UtlRBTreeLinks_t<I>,
	pub data: T,
}

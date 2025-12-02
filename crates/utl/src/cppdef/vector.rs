use ::core::ffi::c_int;

use super::UtlMemory;

#[derive(Debug)]
#[repr(C)]
pub struct UtlVector<T, A = UtlMemory<T>> {
	/// Memory that backs this vector.
	pub memory: A,
	/// Number of initialized elements in this vector.
	pub size: c_int,
	#[cfg(not(feature = "xbox360"))]
	/// Field for easier access to the elements through a debugger.
	pub elements: *mut T,
}

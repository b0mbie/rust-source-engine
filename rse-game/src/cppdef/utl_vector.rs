use ::core::ffi::c_int;

use super::UtlMemory;

#[derive(Debug)]
#[repr(C)]
pub struct UtlVector<T, A = UtlMemory<T>> {
	pub memory: A,
	pub size: c_int,
	#[cfg(not(feature = "xbox"))]
	pub elements: *mut T,
}

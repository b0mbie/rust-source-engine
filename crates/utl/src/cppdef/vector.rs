use ::core::ffi::c_int;

use super::CUtlMemory;

::rse_cpp::class! {
	#[derive(Debug)]
	pub CUtlVector<T, A = CUtlMemory<T>> {
		/// Memory that backs this vector.
		pub memory: A;
		/// Number of initialized elements in this vector.
		pub size: c_int;
		#[cfg(not(feature = "xbox360"))]
		/// Field for easier access to the elements through a debugger.
		pub elements: *mut T;
	}
}

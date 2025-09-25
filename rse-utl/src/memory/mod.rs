use ::core::{
	ffi::c_int,
	mem::MaybeUninit,
};

#[cfg(feature = "tier0")]
pub mod tier0;

mod any_memory;
pub use any_memory::*;
mod grow_size;
pub use grow_size::*;
mod slice;
pub use slice::*;

/// # Safety
/// `n_allocations` must return the correct number of allocations of `T` that can be accessed through `memory`.
pub unsafe trait UtlMemoryOf<T> {
	fn is_externally_allocated(&self) -> bool;
	fn n_allocations(&self) -> usize;
	fn allocations(&self) -> *const MaybeUninit<T>;
}

/// # Safety
/// The pointer returned by `allocations_mut` must be the same as the one returned by [`UtlMemoryOf::allocations`].
pub unsafe trait UtlMemoryOfMut<T>: UtlMemoryOf<T> {
	fn allocations_mut(&mut self) -> *mut MaybeUninit<T>;
}

const fn slice_len_c_int(x: usize) -> c_int {
	if x > (c_int::MAX as usize) {
		c_int::MAX
	} else {
		x as c_int
	}
}

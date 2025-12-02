use ::core::{
	ffi::c_int,
	mem::MaybeUninit,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
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
	#[doc(alias = "NumAllocated")]
	#[doc(alias = "Count")]
	fn n_allocations(&self) -> usize;
	#[doc(alias = "Base")]
	fn allocations(&self) -> *const MaybeUninit<T>;

	/// Returns an immutable slice of the underlying allocations of [`MaybeUninit<T>`].
	fn as_uninit_slice(&self) -> &[MaybeUninit<T>] {
		let ptr = self.allocations();
		if !ptr.is_null() {
			unsafe { from_raw_parts(ptr, self.n_allocations()) }
		} else {
			check_null_allocation(self.n_allocations() as _);
			&[]
		}
	}
}

pub trait UtlMemoryEmpty<T> {
	const EMPTY: Self;
}

/// # Safety
/// The pointer returned by `allocations_mut` must be the same as the one returned by [`UtlMemoryOf::allocations`].
pub unsafe trait UtlMemoryOfMut<T>: UtlMemoryOf<T> {
	#[doc(alias = "Base")]
	fn allocations_mut(&mut self) -> *mut MaybeUninit<T>;

	/// Returns a mutable slice of the underlying allocations of [`MaybeUninit<T>`].
	fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<T>] {
		let ptr = self.allocations_mut();
		if !ptr.is_null() {
			unsafe { from_raw_parts_mut(ptr, self.n_allocations()) }
		} else {
			check_null_allocation(self.n_allocations() as _);
			&mut []
		}
	}
}

pub trait UtlMemoryGrowable<T> {
	type EnsureCapacityError;
	fn ensure_capacity(&mut self, min_capacity: usize) -> Result<(), Self::EnsureCapacityError>;

	type ResizeError;
	fn resize_to(&mut self, new_size: usize) -> Result<(), Self::ResizeError>;
}

const fn is_read_only(grow_size: c_int) -> bool {
	grow_size == crate::cppdef::EXTERNAL_CONST_BUFFER_MARKER
}

const fn check_writable_memory(_grow_size: c_int) {
	#[cfg(any(test, debug_assertions))]
	if is_read_only(_grow_size) {
		panic!("`Memory` function called with read-only slice")
	}
}

const fn check_null_allocation(_allocation_count: c_int) {
	#[cfg(any(test, debug_assertions))]
	if _allocation_count != 0 {
		panic!("allocation count was not 0 for a null allocation")
	}
}

use ::core::{
	ffi::c_int,
	mem::MaybeUninit,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

use crate::cppdef::{
	UtlMemory, EXTERNAL_CONST_BUFFER_MARKER,
};

use super::{
	UtlMemoryOf, GrowSize,
};

/// Transparent wrapper for `CUtlMemory<T>`.
/// 
/// # Layout
/// This type has the exact same layout and ABI as [`UtlMemory<T>`].
#[derive(Debug)]
#[repr(transparent)]
pub struct Memory<T>(UtlMemory<T>);
impl<T> Memory<T> {
	/// Wraps an owned [`UtlMemory<T>`].
	pub const fn new(inner: UtlMemory<T>) -> Self {
		Self(inner)
	}

	/// Returns `true` if the underlying memory is externally allocated and cannot be grown.
	pub const fn is_externally_allocated(&self) -> bool {
		self.0.grow_size < 0
	}

	/// Returns `true` if the underlying memory is externally-allocated, but also read-only.
	pub const fn is_read_only(&self) -> bool {
		is_read_only(self.0.grow_size)
	}

	/// Returns the [`GrowSize`] of the memory if it isn't externally-allocated.
	pub const fn grow_size(&self) -> Option<GrowSize> {
		GrowSize::new(self.0.grow_size)
	}

	/// Returns the number of allocations (in units of `T`) that the memory holds.
	pub const fn n_allocations(&self) -> usize {
		self.0.allocation_count as _
	}

	/// Returns an immutable pointer to the first [`MaybeUninit<T>`] that the underlying memory holds.
	/// 
	/// The pointer may be null.
	pub const fn allocations(&self) -> *const MaybeUninit<T> {
		self.0.memory as _
	}

	/// Returns a mutable pointer to the first [`MaybeUninit<T>`] that the underlying memory holds.
	/// 
	/// The pointer may be null.
	pub const fn allocations_mut(&mut self) -> *mut MaybeUninit<T> {
		check_writable_memory(self.0.grow_size);
		self.0.memory
	}

	/// Returns an immutable slice of the underlying allocations of [`MaybeUninit<T>`].
	pub const fn as_uninit_slice(&self) -> &[MaybeUninit<T>] {
		let ptr = self.0.memory;
		if !ptr.is_null() {
			unsafe { from_raw_parts(ptr, self.n_allocations()) }
		} else {
			check_null_allocation(self.0.allocation_count);
			&[]
		}
	}

	/// Returns a mutable slice of the underlying allocations of [`MaybeUninit<T>`].
	pub const fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<T>] {
		check_writable_memory(self.0.grow_size);
		let ptr = self.0.memory;
		if !ptr.is_null() {
			unsafe { from_raw_parts_mut(ptr, self.n_allocations()) }
		} else {
			check_null_allocation(self.0.allocation_count);
			&mut []
		}
	}

	::rse_cpp::transparent_wrapper_impls!(Memory for UtlMemory<T> as "UtlMemory");
}

const fn is_read_only(grow_size: c_int) -> bool {
	grow_size == EXTERNAL_CONST_BUFFER_MARKER
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

unsafe impl<T> UtlMemoryOf<T> for Memory<T> {
	impl_utl_memory_of! {
		self = self;
		inner = self;
	}
}

macro_rules! impl_utl_memory_of {
	{
		self = $self:ident;
		inner = $inner:expr;
	} => {
		fn is_externally_allocated($self: &Self) -> bool {
			$crate::Memory::<T>::is_externally_allocated($inner)
		}
		fn n_allocations($self: &Self) -> usize {
			$crate::Memory::<T>::n_allocations($inner)
		}
		fn allocations($self: &Self) -> *const ::core::mem::MaybeUninit<T> {
			$crate::Memory::<T>::allocations($inner)
		}
	};
}
pub(crate) use impl_utl_memory_of;

macro_rules! impl_utl_memory_of_mut {
	{
		self = $self:ident;
		inner_mut = $inner_mut:expr;
	} => {
		fn allocations_mut($self: &mut Self) -> *mut ::core::mem::MaybeUninit<T> {
			$crate::Memory::<T>::allocations_mut($inner_mut)
		}
	};
}
pub(crate) use impl_utl_memory_of_mut;

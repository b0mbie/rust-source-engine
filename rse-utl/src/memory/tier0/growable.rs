use ::core::{
	num::NonZeroUsize,
	ptr::null_mut,
};
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	Tier0Allocator,
};

use crate::cppdef::UtlMemory;

use super::{
	super::{
		Memory, GrowSize, slice_len_c_int,
		UtlMemoryOf, impl_utl_memory_of,
		UtlMemoryOfMut, impl_utl_memory_of_mut,
	},
	Tier0Allocatable, Tier0AllocateToken,
};

/// `tier0`-backed, growable [`Memory`].
#[repr(transparent)]
pub struct Tier0Memory<T> {
	memory: Memory<T>,
}

impl<T> Default for Tier0Memory<T>
where
	T: Tier0Allocatable,
{
	fn default() -> Self {
		Self::new(T::TOKEN)
	}
}

impl<T> Tier0Memory<T> {
	pub const fn new(token: Tier0AllocateToken<T>) -> Self {
		Self::with_grow_size(token, GrowSize::DEFAULT)
	}

	pub const fn with_grow_size(token: Tier0AllocateToken<T>, grow_size: GrowSize) -> Self {
		let _ = token;
		Self {
			memory: Memory::new(UtlMemory {
				memory: null_mut(),
				allocation_count: 0,
				grow_size: grow_size.get(),
			}),
		}
	}

	pub fn with_capacity(token: Tier0AllocateToken<T>, capacity: usize) -> Option<Self> {
		if let Some(capacity) = NonZeroUsize::new(capacity) {
			Self::with_nz_capacity(token, capacity)
		} else {
			Some(Self::new(token))
		}
	}

	pub fn with_nz_capacity(token: Tier0AllocateToken<T>, capacity: NonZeroUsize) -> Option<Self> {
		Self::with_nz_cap_and_grow_size(token, capacity, GrowSize::DEFAULT)
	}

	pub fn with_nz_cap_and_grow_size(
		token: Tier0AllocateToken<T>,
		capacity: NonZeroUsize, grow_size: GrowSize,
	) -> Option<Self> {
		let _ = token;
		let allocation_count = slice_len_c_int(capacity.get());
		let memory = unsafe { LinkedTier0Allocator.alloc((allocation_count as usize).checked_mul(size_of::<T>())?) };
		if !memory.is_null() {
			Some(Self {
				memory: Memory::new(UtlMemory {
					memory: memory as _,
					allocation_count,
					grow_size: grow_size.get(),
				})
			})
		} else {
			None
		}
	}

	pub const fn as_inner(&self) -> &Memory<T> {
		&self.memory
	}

	pub const fn as_mut_inner(&mut self) -> &mut Memory<T> {
		&mut self.memory
	}
}

impl<T> Drop for Tier0Memory<T> {
	fn drop(&mut self) {
		let memory = self.memory.allocations_mut();
		if !memory.is_null() {
			unsafe { LinkedTier0Allocator.free(memory as _) }
		}
	}
}

unsafe impl<T> UtlMemoryOf<T> for Tier0Memory<T> {
	impl_utl_memory_of! {
		self = self;
		inner = self.as_inner();
	}
}
unsafe impl<T> UtlMemoryOfMut<T> for Tier0Memory<T> {
	impl_utl_memory_of_mut! {
		self = self;
		inner_mut = self.as_mut_inner();
	}
}

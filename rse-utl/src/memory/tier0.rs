use ::core::{
	marker::PhantomData,
	ptr::null_mut,
};
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	Tier0Allocator, can_be_aligned,
};

use crate::cppdef::UtlMemory;

use super::{
	Memory, GrowSize, slice_len_c_int,
	UtlMemoryOf, impl_utl_memory_of,
	UtlMemoryOfMut, impl_utl_memory_of_mut,
};

pub const fn tier0_memory_provider<T>() -> Option<Tier0MemoryProvider<T>> {
	Tier0MemoryProvider::<T>::INSTANCE
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Tier0MemoryProvider<T>(PhantomData<fn(T)>);
impl<T> Tier0MemoryProvider<T> {
	pub const INSTANCE: Option<Self> = if can_be_aligned::<T>() {
		Some(Self(PhantomData))
	} else {
		None
	};

	pub const fn default(self) -> Tier0Memory<T> {
		self.create(GrowSize::DEFAULT)
	}

	pub const fn create(self, grow_size: GrowSize) -> Tier0Memory<T> {
		Tier0Memory {
			memory: Memory::new(UtlMemory {
				memory: null_mut(),
				allocation_count: 0,
				grow_size: grow_size.get(),
			}),
		}
	}

	pub fn allocate(self, capacity: usize) -> Option<Tier0Memory<T>> {
		self.allocate_with(capacity, GrowSize::DEFAULT)
	}

	pub fn allocate_with(self, capacity: usize, grow_size: GrowSize) -> Option<Tier0Memory<T>> {
		let allocation_count = slice_len_c_int(capacity);
		let memory = unsafe { LinkedTier0Allocator.alloc((allocation_count as usize).checked_mul(size_of::<T>())?) };
		if !memory.is_null() {
			Some(Tier0Memory {
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
}

/// `tier0`-backed, growable [`Memory`].
#[repr(transparent)]
pub struct Tier0Memory<T> {
	memory: Memory<T>,
}

impl<T> Tier0Memory<T> {
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

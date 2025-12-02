use ::core::{
	num::NonZeroUsize,
	ptr::null_mut,
};
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	Tier0Allocator,
};

use crate::{
	cppdef::UtlMemory,
	util::clamp_len_to_c_int,
};

use super::{
	super::{
		Memory, GrowSize,
		UtlMemoryEmpty,
		UtlMemoryOf, impl_utl_memory_of,
		UtlMemoryOfMut, impl_utl_memory_of_mut,
		UtlMemoryGrowable,
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
		let allocation_count = clamp_len_to_c_int(capacity.get());
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

	const fn item_size(&self) -> NonZeroUsize {
		unsafe { NonZeroUsize::new_unchecked(size_of::<T>()) }
	}

	#[doc(alias = "Purge")]
	pub fn clear(&mut self) {
		let memory = &mut self.memory;
		if !memory.is_externally_allocated() {
			let inner = unsafe { memory.as_mut_inner() };
			let ptr = inner.memory;
			if !ptr.is_null() {
				// UTLMEMORY_TRACK_FREE
				unsafe { LinkedTier0Allocator.free(ptr as _) }
				inner.memory = null_mut();
			}
			inner.allocation_count = 0;
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
		self.clear()
	}
}

impl<T> UtlMemoryEmpty<T> for Tier0Memory<T>
where
	T: Tier0Allocatable,
{
	const EMPTY: Self = Self::new(T::TOKEN);
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

impl<T> UtlMemoryGrowable<T> for Tier0Memory<T> {
	type EnsureCapacityError = EnsureCapacityError;
	fn ensure_capacity(&mut self, min_capacity: usize) -> Result<(), Self::EnsureCapacityError> {
		let inner = &mut self.memory;
		let n_allocations = inner.n_allocations();
		if n_allocations >= min_capacity {
			return Ok(())
		}

		if inner.is_externally_allocated() {
			return Err(EnsureCapacityError::ExternallyAllocated)
		}

		// UTLMEMORY_TRACK_FREE

		// UTLMEMORY_TRACK_ALLOC
		// MEM_ALLOC_CREDIT_CLASS

		let raw = unsafe { inner.as_mut_inner() };
		let ptr = raw.memory;
		
		let alloc_size = n_allocations * size_of::<T>();
		let new_ptr = if !ptr.is_null() {
			unsafe { LinkedTier0Allocator.realloc(ptr as _, alloc_size) }
		} else {
			unsafe { LinkedTier0Allocator.alloc(alloc_size) }
		};
		
		if !new_ptr.is_null() {
			raw.memory = new_ptr as _;
			raw.allocation_count = clamp_len_to_c_int(min_capacity);
			Ok(())
		} else {
			Err(EnsureCapacityError::AllocError)
		}
	}

	type ResizeError = ResizeError;
	fn resize_to(&mut self, new_size: usize) -> Result<(), Self::ResizeError> {
		let item_size = self.item_size();
		let inner = &mut self.memory;
		let grow_size = inner.grow_size().ok_or(ResizeError::ExternallyAllocated)?;

		let new_allocation_count = calc_new_allocation_count(
			inner.n_allocations(), new_size,
			grow_size, item_size,
		).ok_or(ResizeError::TooBig)?;

		// FIXME: `new_allocation_count` gets clamped without warning. This may or may not be bad!
		let new_allocation_count = clamp_len_to_c_int(new_allocation_count);

		let ptr = inner.allocations_mut();
		
		// UTLMEMORY_TRACK_FREE
		// UTLMEMORY_TRACK_ALLOC
		// MEM_ALLOC_CREDIT_CLASS
		let alloc_size = ((new_allocation_count as usize).checked_mul(item_size.get())).ok_or(ResizeError::TooBig)?;
		let new_ptr = if !ptr.is_null() {
			unsafe { LinkedTier0Allocator.realloc(ptr as _, alloc_size) }
		} else {
			unsafe { LinkedTier0Allocator.alloc(alloc_size) }
		};

		if !new_ptr.is_null() {
			let inner = unsafe { inner.as_mut_inner() };
			inner.memory = new_ptr as _;
			inner.allocation_count = new_allocation_count;
			Ok(())
		} else {
			Err(ResizeError::TooBig)
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum EnsureCapacityError {
	#[error("memory is externally allocated and cannot be grown")]
	ExternallyAllocated,
	#[error("couldn't allocate memory")]
	AllocError,
}

#[derive(Debug, thiserror::Error)]
pub enum ResizeError {
	#[error("memory is externally allocated and cannot be grown")]
	ExternallyAllocated,
	#[error("there is not enough memory to grow the memory")]
	TooBig,
	#[error("couldn't allocate memory")]
	AllocError,
}

const fn calc_new_allocation_count(
	old_size: usize, new_size: usize,
	grow_size: GrowSize, item_size: NonZeroUsize,
) -> Option<usize> {
	macro_rules! unwrap {
		($opt:expr) => {
			match $opt {
				Some(t) => t,
				None => return None,
			}
		};
	}

	let grow_size = grow_size.get() as usize;
	if grow_size > 0 {
		Some({
			let stepped = new_size.next_multiple_of(grow_size);
			if stepped < grow_size {
				grow_size
			} else {
				stepped
			}
		})
	} else {
		let mut result = if old_size == 0 {
			unwrap!(item_size.checked_add(31)).get() / item_size.get()
		} else {
			old_size
		};

		while result < new_size {
			if cfg!(not(feature = "xbox360")) {
				result = unwrap!(result.checked_mul(2));
			} else {
				// Add 12.5% to size.
				let try_size = unwrap!(unwrap!(result.checked_mul(9)).checked_div(8));

				// This will always go to the `true` branch. So, it is unused.
				/*
				if try_size > result {
					result = try_size;
				} else {
					result = unwrap!(result.checked_mul(2));
				}
				*/

				result = try_size;
			}
		}

		Some(result)
	}
}

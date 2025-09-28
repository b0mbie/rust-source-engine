use ::core::{
	marker::PhantomData,
	mem::MaybeUninit,
	ptr::null_mut,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

use crate::{
	cppdef::{
		UtlMemory,
		EXTERNAL_BUFFER_MARKER, EXTERNAL_CONST_BUFFER_MARKER,
	},
	util::clamp_len_to_c_int,
};

use super::{
	Memory,
	UtlMemoryEmpty,
	UtlMemoryOf, impl_utl_memory_of,
	UtlMemoryOfMut, impl_utl_memory_of_mut,
	UtlMemoryGrowable,
};

/// Returns a [`SliceMemory`] backed by an immutable slice of `T`.
pub const fn mem_of<T>(slice: &[T]) -> SliceMemory<T, &[MaybeUninit<T>]> {
	SliceMemory::from_slice(slice)
}

/// Returns a [`SliceMemory`] backed by a mutable slice of `T`.
pub const fn mem_of_mut<T>(slice: &mut [T]) -> SliceMemory<T, &mut [MaybeUninit<T>]> {
	SliceMemory::from_mut_slice(slice)
}

/// Slice-backed, non-growable [`Memory`].
#[derive(Debug)]
#[repr(transparent)]
pub struct SliceMemory<T, S> {
	memory: Memory<T>,
	_slice: PhantomData<S>,
}

impl<T, S> SliceMemory<T, S> {
	/// Returns an empty slice-backed memory.
	pub const fn empty() -> Self {
		Self {
			memory: Memory::new(UtlMemory {
				memory: null_mut(),
				allocation_count: 0,
				grow_size: EXTERNAL_BUFFER_MARKER,
			}),
			_slice: PhantomData,
		}
	}

	/// Returns an immutable reference to the inner [`Memory<T>`].
	pub const fn as_memory(&self) -> &Memory<T> {
		&self.memory
	}

	/// Returns a mutable reference to the inner [`Memory<T>`],
	/// without checking if the memory is mutable.
	/// 
	/// # Safety
	/// `S` must point to mutable memory.
	pub const unsafe fn as_mut_memory_unchecked(&mut self) -> &mut Memory<T> {
		&mut self.memory
	}
}

impl<'a, T> SliceMemory<T, &'a [MaybeUninit<T>]> {
	/// Returns an immutable slice-backed memory backed by a slice of `T`.
	pub const fn from_slice(slice: &'a [T]) -> Self {
		let uninit_slice = unsafe { from_raw_parts(slice.as_ptr() as *const MaybeUninit<T>, slice.len()) };
		Self::from_uninit_slice(uninit_slice)
	}

	/// Returns an immutable slice-backed memory backed by a slice of [`MaybeUninit<T>`].
	pub const fn from_uninit_slice(slice: &'a [MaybeUninit<T>]) -> Self {
		Self {
			memory: Memory::new(UtlMemory {
				memory: slice.as_ptr() as _,
				allocation_count: clamp_len_to_c_int(slice.len()),
				grow_size: EXTERNAL_CONST_BUFFER_MARKER,
			}),
			_slice: PhantomData,
		}
	}

	/// Returns the contents of the memory as a slice of [`MaybeUninit<T>`].
	pub const fn as_slice(&self) -> &'a [MaybeUninit<T>] {
		unsafe { from_raw_parts(self.memory.allocations(), self.memory.n_allocations()) }
	}
}

impl<'a, T> SliceMemory<T, &'a mut [MaybeUninit<T>]> {
	/// Returns a mutable slice-backed memory backed by a slice of `T`.
	pub const fn from_mut_slice(slice: &'a mut [T]) -> Self {
		let uninit_slice = unsafe { from_raw_parts_mut(slice.as_ptr() as *mut MaybeUninit<T>, slice.len()) };
		Self::from_uninit_slice_mut(uninit_slice)
	}

	/// Returns a mutable slice-backed memory backed by a slice of [`MaybeUninit<T>`].
	pub const fn from_uninit_slice_mut(slice: &'a mut [MaybeUninit<T>]) -> Self {
		Self {
			memory: Memory::new(UtlMemory {
				memory: slice.as_mut_ptr(),
				allocation_count: clamp_len_to_c_int(slice.len()),
				grow_size: EXTERNAL_BUFFER_MARKER,
			}),
			_slice: PhantomData,
		}
	}

	/// Returns the contents of the memory as a slice of [`MaybeUninit<T>`].
	pub const fn as_slice(&self) -> &'a [MaybeUninit<T>] {
		unsafe { from_raw_parts(self.memory.allocations(), self.memory.n_allocations()) }
	}

	/// Returns the mutable contents of the memory as a slice of [`MaybeUninit<T>`].
	pub const fn as_mut_slice(&mut self) -> &'a mut [MaybeUninit<T>] {
		unsafe { from_raw_parts_mut(self.memory.allocations_mut(), self.memory.n_allocations()) }
	}

	/// Returns a mutable reference to the inner [`Memory<T>`].
	pub const fn as_mut_memory(&mut self) -> &mut Memory<T> {
		unsafe { self.as_mut_memory_unchecked() }
	}
}

impl<T, S> UtlMemoryEmpty<T> for SliceMemory<T, S> {
	const EMPTY: Self = Self::empty();
}
unsafe impl<T, S> UtlMemoryOf<T> for SliceMemory<T, S> {
	impl_utl_memory_of! {
		self = self;
		inner = self.as_memory();
	}
}
unsafe impl<T> UtlMemoryOfMut<T> for SliceMemory<T, &mut [MaybeUninit<T>]> {
	impl_utl_memory_of_mut! {
		self = self;
		inner_mut = self.as_mut_memory();
	}
}
impl<T, S> UtlMemoryGrowable<T> for SliceMemory<T, S> {
	type EnsureCapacityError = SliceGrowError;
	fn ensure_capacity(&mut self, min_capacity: usize) -> Result<(), Self::EnsureCapacityError> {
		if self.memory.n_allocations() >= min_capacity {
			Ok(())
		} else {
			Err(SliceGrowError)
		}
	}

	type ResizeError = SliceGrowError;
	fn resize_to(&mut self, new_size: usize) -> Result<(), Self::ResizeError> {
		if new_size <= self.memory.n_allocations() {
			unsafe { self.memory.as_mut_inner().allocation_count = clamp_len_to_c_int(new_size) };
			Ok(())
		} else {
			Err(SliceGrowError)
		}
	}
}

#[derive(Debug, thiserror::Error)]
#[error("slice-backed memory cannot be grown")]
pub struct SliceGrowError;

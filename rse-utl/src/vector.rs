use ::alloc::alloc::Layout;
use ::core::{
	fmt,
	ptr::null_mut,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

use crate::{
	cppdef::UtlVector,
	util::clamp_len_to_c_int,
};

use super::memory::{
	UtlMemoryEmpty, UtlMemoryOfMut, UtlMemoryGrowable,
};

/// Transparent wrapper for `CUtlVector<T>`.
/// 
/// The type parameter `A` must implement [`UtlMemoryOfMut<T>`].
/// When the feature `tier0` is enabled, the default for `A` is `Tier0Memory<T>`.
/// 
/// # Layout
/// This type has the exact same layout and ABI as [`UtlVector<T, A>`].
#[repr(transparent)]
pub struct Vector<
	T,
	#[cfg(feature = "tier0")]
	A = super::memory::tier0::Tier0Memory<T>,
	#[cfg(not(feature = "tier0"))]
	A,
>(UtlVector<T, A>)
where
	A: UtlMemoryOfMut<T>;

impl<T, A> Drop for Vector<T, A>
where
	A: UtlMemoryOfMut<T>,
{
	fn drop(&mut self) {
		let len = self.len();
		let allocations = self.0.memory.as_uninit_slice_mut();
		debug_assert!(len <= allocations.len(), "vector length can never be greater than its capacity");
		unsafe {
			for t in allocations.get_unchecked_mut(..len) {
				t.assume_init_drop();
			}
		}
	}
}

impl<T, A> Vector<T, A>
where
	A: UtlMemoryOfMut<T>,
{
	pub const fn empty() -> Self
	where
		A: UtlMemoryEmpty<T>,
	{
		Self(UtlVector {
			memory: A::EMPTY,
			size: 0,
			#[cfg(not(feature = "xbox360"))]
			elements: null_mut(),
		})
	}

	/// Returns a new vector with the given underlying `memory`.
	pub fn new(memory: A) -> Self {
		let elements = memory.allocations() as _;
		Self(UtlVector {
			memory,
			size: 0,
			#[cfg(not(feature = "xbox360"))]
			elements,
		})
	}

	/// Refresh information used for debugging.
	pub fn reset_dbg_info(&mut self) {
		#[cfg(not(feature = "xbox360"))]
		{
			self.0.elements = self.0.memory.allocations() as _;
		}
	}

	/// Returns the capacity of the vector.
	pub fn capacity(&self) -> usize {
		self.0.memory.n_allocations()
	}

	/// Returns the length of the vector.
	pub const fn len(&self) -> usize {
		self.0.size as _
	}

	/// Directly sets the length of the vector.
	/// 
	/// # Safety
	/// The indices that `len` frees must be already initialized.
	pub const unsafe fn set_len(&mut self, len: usize) {
		self.0.size = clamp_len_to_c_int(len);
	}

	/// Returns `true` if the vector is empty.
	pub const fn is_empty(&self) -> bool {
		self.0.size == 0
	}

	/// Returns an immutable slice of the vector's contents.
	pub fn as_slice(&self) -> &[T] {
		let ptr = self.0.memory.allocations() as *const T;
		if !ptr.is_null() {
			unsafe { from_raw_parts(ptr, self.len()) }
		} else {
			&[]
		}
	}

	/// Returns a mutable slice of the vector's contents.
	pub fn as_mut_slice(&mut self) -> &mut [T]
	where
		A: UtlMemoryOfMut<T>,
	{
		let ptr = self.0.memory.allocations_mut() as *mut T;
		if !ptr.is_null() {
			unsafe { from_raw_parts_mut(self.0.memory.allocations_mut() as *mut T, self.len()) }
		} else {
			&mut []
		}
	}

	/// Returns an immutable reference to the underlying memory allocator.
	pub const fn memory(&self) -> &A {
		&self.0.memory
	}

	/// Returns a mutable reference to the underlying memory allocator.
	pub const fn memory_mut(&mut self) -> &mut A {
		&mut self.0.memory
	}

	::rse_cpp::transparent_wrapper_impls!(Vector for UtlVector<T, A> as "UtlVector");

	#[doc(alias = "AddToTail")]
	pub fn push(&mut self, value: T)
	where
		A: UtlMemoryGrowable<T>,
	{
		match self.try_push_impl(value) {
			Ok(t) => t,
			Err(TryPushImplError::TooMuch { .. }) => panic!("vector memory cannot hold any more elements"),
			Err(TryPushImplError::EnsureCapacity { layout, .. }) => {
				::alloc::alloc::handle_alloc_error(layout)
			}
		}
	}

	#[doc(alias = "AddToTail")]
	pub fn try_push(&mut self, value: T) -> Result<(), TryPushError<T>>
	where
		A: UtlMemoryGrowable<T>,
	{
		match self.try_push_impl(value) {
			Ok(t) => Ok(t),
			Err(TryPushImplError::TooMuch { value }) => {
				Err(TryPushError {
					value,
					kind: TryPushErrorKind::TooMuch,
				})
			}
			Err(TryPushImplError::EnsureCapacity { value, error, .. }) => {
				Err(TryPushError {
					value,
					kind: TryPushErrorKind::EnsureCapacity(error),
				})
			}
		}
	}

	fn try_push_impl(&mut self, value: T) -> Result<(), TryPushImplError<T>>
	where
		A: UtlMemoryGrowable<T>,
	{
		let len = self.len();
		let (new_len, layout) = {
			let result = match len.checked_add(1) {
				Some(new_len) => match Layout::array::<T>(new_len) {
					Ok(layout) => Some((new_len, layout)),
					Err(..) => None,
				}
				None => None,
			};
			match result {
				Some(t) => t,
				None => {
					return Err(TryPushImplError::TooMuch { value, })
				}
			}
		};

		match self.ensure_capacity(new_len) {
			Ok(..) => unsafe {
				self.0.memory.as_uninit_slice_mut().get_unchecked_mut(len).write(value);
				self.set_len(new_len);
				Ok(())
			}
			Err(error) => Err(TryPushImplError::EnsureCapacity {
				value,
				error,
				layout,
			}),
		}
	}

	pub fn ensure_capacity(&mut self, new_capacity: usize) -> Result<(), EnsureCapacityError>
	where
		A: UtlMemoryGrowable<T>,
	{
		if new_capacity > self.capacity() {
			// MEM_ALLOC_CREDIT_CLASS
			let is_ok = self.0.memory.resize_to(new_capacity).is_ok();
			self.reset_dbg_info();
			if is_ok {
				Ok(())
			} else {
				Err(EnsureCapacityError)
			}
		} else {
			self.reset_dbg_info();
			Ok(())
		}
	}

	#[doc(alias = "EnsureCapacity")]
	pub fn ensure_capacity_exact(&mut self, min_capacity: usize) -> Result<(), EnsureCapacityExactError>
	where
		A: UtlMemoryGrowable<T>,
	{
		let is_ok = self.0.memory.ensure_capacity(min_capacity).is_ok();
		self.reset_dbg_info();
		if is_ok { Ok(()) } else { Err(EnsureCapacityExactError) }
	}
}

impl<T, A> fmt::Debug for Vector<T, A>
where
	A: UtlMemoryOfMut<T>,
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self.as_slice(), f)
	}
}

enum TryPushImplError<T> {
	TooMuch {
		value: T,
	},
	EnsureCapacity {
		value: T,
		error: EnsureCapacityError,
		layout: Layout,
	},
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct TryPushError<T> {
	pub value: T,
	pub kind: TryPushErrorKind,
}

#[derive(Debug, thiserror::Error)]
pub enum TryPushErrorKind {
	#[error("vector memmory cannot hold any more elements")]
	TooMuch,
	#[error("{0}")]
	EnsureCapacity(EnsureCapacityError),
}

#[derive(Debug, thiserror::Error)]
#[error("couldn't grow vector memory to requested size")]
pub struct EnsureCapacityError;

#[derive(Debug, thiserror::Error)]
#[error("vector memory cannot provide the requested capacity")]
pub struct EnsureCapacityExactError;

#[test]
fn try_push() {
	use ::core::mem::MaybeUninit;
	use crate::memory::SliceMemory;
	let mut memory = [MaybeUninit::uninit(); 16];
	let mut vector = Vector::<u8, SliceMemory<u8, &mut [MaybeUninit<u8>]>>::new(
		SliceMemory::from_uninit_slice_mut(&mut memory)
	);

	for i in 1..=vector.capacity() as u8 {
		assert!(vector.try_push(i).is_ok());
	}
	assert!(vector.try_push(255).is_err());
}

#[test]
fn drop_elements() {
	use ::core::{
		mem::MaybeUninit,
		sync::atomic::{
			AtomicUsize, Ordering,
		},
	};
	use crate::memory::SliceMemory;

	#[non_exhaustive]
	struct DropCounter;
	static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);
	impl DropCounter {
		pub fn new() -> Self {
			DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
			Self
		}
	}
	impl Drop for DropCounter {
		fn drop(&mut self) {
			DROP_COUNTER.fetch_sub(1, Ordering::SeqCst);
		}
	}

	const CAPACITY: usize = 16;
	const N_FILLED: usize = 7;

	let mut uninit_array = MaybeUninit::<[DropCounter; CAPACITY]>::uninit();
	let memory: &mut [MaybeUninit<DropCounter>] = {
		unsafe { from_raw_parts_mut(uninit_array.as_mut_ptr() as *mut MaybeUninit<DropCounter>, CAPACITY) }
	};
	assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 0);

	let mut vector = Vector::<DropCounter, SliceMemory<DropCounter, &mut [MaybeUninit<DropCounter>]>>::new(
		SliceMemory::from_uninit_slice_mut(memory)
	);

	for _ in 0..N_FILLED {
		assert!(vector.try_push(DropCounter::new()).is_ok());
	}
	assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), N_FILLED);

	drop(vector);
	assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 0);
}

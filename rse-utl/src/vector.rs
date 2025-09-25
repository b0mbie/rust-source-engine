use ::core::{
	fmt,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

use crate::cppdef::UtlVector;

use super::memory::{
	UtlMemoryOf, UtlMemoryOfMut,
};

/// Transparent wrapper for `CUtlVector<T>`.
/// 
/// The type parameter `A` must implement [`UtlMemoryOf<T>`].
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
>(UtlVector<T, A>);

impl<T, A> Vector<T, A>
where
	A: UtlMemoryOf<T>,
{
	pub fn new(memory: A) -> Self {
		let elements = memory.allocations() as _;
		Self(UtlVector {
			memory,
			size: 0,
			#[cfg(not(feature = "xbox360"))]
			elements,
		})
	}

	/// Returns the length of the vector.
	pub const fn len(&self) -> usize {
		self.0.size as _
	}

	/// Returns `true` if the vector is empty.
	pub const fn is_empty(&self) -> bool {
		self.0.size == 0
	}

	/// Returns an immutable slice of the vector's contents.
	pub fn as_slice(&self) -> &[T] {
		unsafe { from_raw_parts(self.0.memory.allocations() as *const T, self.len()) }
	}

	/// Returns a mutable slice of the vector's contents.
	pub fn as_mut_slice(&mut self) -> &mut [T]
	where
		A: UtlMemoryOfMut<T>,
	{
		unsafe { from_raw_parts_mut(self.0.memory.allocations_mut() as *mut T, self.len()) }
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
}

impl<T, A> fmt::Debug for Vector<T, A>
where
	A: UtlMemoryOf<T>,
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self.as_slice(), f)
	}
}

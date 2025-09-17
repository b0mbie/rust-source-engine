use ::core::{
	ffi::c_int,
	mem::MaybeUninit,
};

pub const EXTERNAL_BUFFER_MARKER: c_int = -1;
pub const EXTERNAL_CONST_BUFFER_MARKER: c_int = -2;

#[derive(Debug)]
#[repr(C)]
pub struct UtlMemory<T> {
	/// Pointer to the possibly-uninitialized, possibly-unaligned allocations of `T`.
	/// 
	/// The number of allocations is determined by `allocation_count`.
	pub memory: *mut MaybeUninit<T>,
	/// Number of allocations of `T` that `memory` holds.
	pub allocation_count: c_int,
	/// Value used for growing the memory.
	/// 
	/// This value has different meanings.
	/// Values `> 0` indicate the size of the "step" to grow the `allocation_count` by,
	/// a value of `0` requests exponential growth,
	/// and values `< 0` indicate that the memory is of a static size and cannot be grown,
	/// since it was externally allocated.
	/// More specifically,
	/// [`EXTERNAL_BUFFER_MARKER`] indicates mutable external memory, and
	/// [`EXTERNAL_CONST_BUFFER_MARKER`] indicates immutable external memory.
	/// 
	/// For example, given `grow_size = 16`:
	/// - a request to allocate `0` will yield `allocation_count = 0`,
	/// - a request to allocate `1` will yield `allocation_count = 16`,
	/// - a request to allocate `16` will yield `allocation_count = 16`, and
	/// - a request to allocate `17` will yield `allocation_count = 32`.
	pub grow_size: c_int,
}

use ::core::ffi::{
	CStr, c_int,
};

pub const TIER0_MIN_ALIGN: usize = align_of::<::libc::max_align_t>();

pub const fn can_be_aligned<T>() -> bool {
	align_of::<T>() <= TIER0_MIN_ALIGN
}

pub trait Tier0Alloc {
	type Allocator<'a>: Tier0Allocator where Self: 'a;
	fn allocator(&self) -> Self::Allocator<'_>;
}

/// # Safety
/// The allocator functions must align like typical C/C++ allocators; that is:
/// - the alignment *must* be a power of two, and
/// - any alignment less than or equal to [`TIER0_MIN_ALIGN`] must be implicitly supported.
pub unsafe trait Tier0Allocator {
	/// # Safety
	/// `size` must be non-zero.
	/// 
	/// The block of memory returned by this function may or may not be initialized.
	unsafe fn alloc(&self, size: usize) -> *mut u8;
	/// # Safety
	/// `new_size` must be non-zero.
	/// 
	/// Additionally, the block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn realloc(&self, mem: *mut u8, new_size: usize) -> *mut u8;
	/// # Safety
	/// The block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn free(&self, mem: *mut u8);

	/// Like [`alloc`](Tier0Allocator::alloc),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// `size` must be non-zero.
	/// 
	/// The block of memory returned by this function may or may not be initialized.
	unsafe fn debug_alloc(&self, size: usize, loc: Location<'_>) -> *mut u8;
	/// Like [`realloc`](Tier0Allocator::realloc),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// `new_size` must be non-zero.
	/// 
	/// Additionally, the block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn debug_realloc(&self, mem: *mut u8, new_size: usize, loc: Location<'_>) -> *mut u8;
	/// Like [`free`](Tier0Allocator::free),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// The block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn debug_free(&self, mem: *mut u8, loc: Location<'_>);

	/// # Safety
	/// The block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn size_of(&self, mem: *mut u8) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location<'a> {
	pub filename: &'a CStr,
	pub line: c_int,
}

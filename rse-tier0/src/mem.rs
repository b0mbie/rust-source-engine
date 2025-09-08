use ::core::ffi::{
	CStr, c_int,
};

pub trait Tier0Alloc {
	type Allocator<'a>: Tier0Allocator where Self: 'a;
	fn allocator(&self) -> Self::Allocator<'_>;
}

pub trait Tier0Allocator {
	/// # Safety
	/// `size` must be non-zero.
	/// 
	/// The block of memory returned by this function may or may not be initialized.
	unsafe fn alloc_unaligned(&self, size: usize) -> *mut u8;
	/// # Safety
	/// `new_size` must be non-zero.
	/// 
	/// Additionally, the block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn realloc_unaligned(&self, mem: *mut u8, new_size: usize) -> *mut u8;
	/// # Safety
	/// The block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn free_unaligned(&self, mem: *mut u8);

	/// Like [`alloc_unaligned`](Tier0Alloc::alloc_unaligned),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// `size` must be non-zero.
	/// 
	/// The block of memory returned by this function may or may not be initialized.
	unsafe fn debug_alloc_unaligned(&self, size: usize, loc: Location<'_>) -> *mut u8;
	/// Like [`realloc_unaligned`](Tier0Alloc::realloc_unaligned),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// `new_size` must be non-zero.
	/// 
	/// Additionally, the block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn debug_realloc_unaligned(&self, mem: *mut u8, new_size: usize, loc: Location<'_>) -> *mut u8;
	/// Like [`free_unaligned`](Tier0Alloc::free_unaligned),
	/// but allows for specifying a [`Location`] in code for debugging purposes.
	/// 
	/// # Safety
	/// The block of memory pointed to by `mem`:
	/// - must have been allocated with the implementing allocator, and
	/// - must not have been freed.
	unsafe fn debug_free_unaligned(&self, mem: *mut u8, loc: Location<'_>);

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

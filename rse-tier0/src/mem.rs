use ::core::{
	alloc::{
		Layout, GlobalAlloc,
	},
	ffi::{
		CStr, c_int,
	},
	ptr::null_mut,
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

/// Wrapper for [`Tier0Allocator`] unaligned allocators to implement [`GlobalAlloc`].
/// 
/// Unaligned allocators are allocators which typically always work fine for allocations of alignment `1`.
/// For types that have an alignment of more than `1` (like [`i32`]),
/// there needs to be enough padding for the allocation
/// so that the original allocation pointer can be accessed again (for reallocating/deallocating)
/// while also providing the correct alignment.
/// 
/// Unlike what the utilities provided the by the public `tier0` implementation can have,
/// Rust will *always* provide a `Layout` for Rust allocators,
/// so the parameters for an allocation can be determined
/// such that there is no need to store extra information inside of the allocation itself -
/// besides padding, which will be used to align the allocated object.
/// 
/// This incurs a small allocated block size penalty that is equal to `alignment - 1`;
/// types that are aligned like [`u8`] need no additional padding,
/// and arrays of those types are also allocated without the need for padding.
#[repr(transparent)]
pub struct Tier0GlobalAlloc<A: ?Sized>(pub A);

unsafe impl<A> GlobalAlloc for Tier0GlobalAlloc<A>
where
	A: ?Sized + Tier0Allocator,
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let (size, alignment) = (layout.size(), layout.align());
		// SAFETY: Due to how Rust's type system works, `align >= 1`.
		let align_space = unsafe { alignment.unchecked_sub(1) };
		let allocated_size = align_space + size;
		let pointer = unsafe { self.0.alloc_unaligned(allocated_size) };
		if pointer.is_null() {
			return null_mut()
		}

		unsafe {
			let misalignment = pointer.align_offset(alignment);
			pointer.byte_add(misalignment)
		}
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe { self.0.free_unaligned(unaligned_ptr(&self.0, ptr, layout)) }
	}
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		unsafe { self.0.realloc_unaligned(unaligned_ptr(&self.0, ptr, layout), new_size) }
	}
}

/// # Safety
/// `layout` must be the same layout that was used to allocate the block of memory `mem`,
/// and it must've come from an allocation done by the `allocator`.
// Debug assertions are used here because of the safety conditions existing.
unsafe fn unaligned_ptr<A: ?Sized + Tier0Allocator>(allocator: &A, mem: *mut u8, layout: Layout) -> *mut u8 {
	let (size, alignment) = (layout.size(), layout.align());

	debug_assert!(
		is_aligned_to(mem, alignment),
		"`Tier0Alloc` aligned routines must be called with well-aligned pointers",
	);

	let allocated_size = unsafe { allocator.size_of(mem) };
	debug_assert!(
		allocated_size >= size,
		"`Tier0Alloc` aligned routines must be called with a `layout` matching the allocation",
	);
	let misalignment = unsafe { allocated_size.unchecked_sub(size) };

	unsafe { mem.byte_sub(misalignment) }
}

fn is_aligned_to(ptr: *mut u8, alignment: usize) -> bool {
	ptr.addr() & (alignment - 1) == 0
}

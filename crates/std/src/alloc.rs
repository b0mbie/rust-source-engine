use ::core::alloc::Layout;
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	mem_alloc,
	Tier0Allocator,
};

/// # Safety
/// `layout` must have non-zero size.
/// Attempting to allocate for a zero-sized `layout` may result in undefined behavior.
/// 
/// The returned block of memory may or may not be initialized.
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
	unsafe { mem_alloc::alloc(&LinkedTier0Allocator, layout) }
}

/// # Safety
/// The caller must ensure that:
/// - `ptr` is a block of memory currently allocated via this allocator, and
/// - `layout` is the same layout that was used to allocate that block of memory.
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
	unsafe { mem_alloc::dealloc(&LinkedTier0Allocator, ptr, layout) }
}

/// # Safety
/// The caller must ensure that:
/// - `ptr` is allocated via this allocator,
/// - `layout` is the same layout that was used to allocate that block of memory,
/// - `new_size` is greater than zero, and
/// - `new_size`, when rounded up to the nearest multiple of `layout.align()`,
///   does not overflow [`isize`]
///   (i.e. the rounded value must be less than or equal to [`isize::MAX`]).
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
	unsafe { mem_alloc::realloc(&LinkedTier0Allocator, ptr, layout, new_size) }
}

/// # Safety
/// `size` must be non-zero.
/// 
/// The block of memory returned by this function may or may not be initialized.
pub unsafe fn alloc_unaligned(size: usize) -> *mut u8 {
	unsafe { LinkedTier0Allocator.alloc(size) }
}

/// # Safety
/// The block of memory pointed to by `mem`:
/// - must have been allocated with the implementing allocator, and
/// - must not have been freed.
pub unsafe fn dealloc_unaligned(ptr: *mut u8) {
	unsafe { LinkedTier0Allocator.free(ptr) }
}

/// # Safety
/// `new_size` must be non-zero.
/// 
/// Additionally, the block of memory pointed to by `mem`:
/// - must have been allocated with the implementing allocator, and
/// - must not have been freed.
pub unsafe fn realloc_unaligned(ptr: *mut u8, new_size: usize) -> *mut u8 {
	unsafe { LinkedTier0Allocator.realloc(ptr, new_size) }
}

#[cfg(feature = "global-allocator")]
mod global_allocator {
	use ::rse_tier0::{
		linked::mem::LinkedTier0Allocator,
		mem_alloc::Tier0GlobalAlloc,
	};
	#[global_allocator]
	static ALLOCATOR: Tier0GlobalAlloc<LinkedTier0Allocator> = Tier0GlobalAlloc(LinkedTier0Allocator);
}

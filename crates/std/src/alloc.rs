use ::core::alloc::Layout;
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	mem_alloc,
	Tier0Allocator,
};

pub unsafe fn alloc(layout: Layout) -> *mut u8 {
	unsafe { mem_alloc::alloc(&LinkedTier0Allocator, layout) }
}

pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
	unsafe { mem_alloc::dealloc(&LinkedTier0Allocator, ptr, layout) }
}

pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
	unsafe { mem_alloc::realloc(&LinkedTier0Allocator, ptr, layout, new_size) }
}

pub unsafe fn alloc_unaligned(size: usize) -> *mut u8 {
	unsafe { LinkedTier0Allocator.alloc(size) }
}

pub unsafe fn dealloc_unaligned(ptr: *mut u8) {
	unsafe { LinkedTier0Allocator.free(ptr) }
}

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

use ::alloc::alloc::{
	GlobalAlloc, Layout,
};
use ::core::{
	num::NonZeroUsize,
	ptr::null_mut,
};

use crate::{
	Tier0Allocator, TIER0_MIN_ALIGN,
};

macro_rules! check_aligned {
	($method:literal; $align:expr, $ptr:expr) => {{
		let ptr = $ptr;
		debug_assert_eq!(
			ptr.align_offset($align), 0,
			concat!(
				"safety invariant for `Tier0Allocator` was not upheld: ",
				"call to '", $method, "' returned unaligned pointer",
			),
		);
		ptr
	}};
}

/// Allocates memory as described by the given `layout`,
/// returning a pointer to the newly-allocated memory,
/// or null to indicate failure.
/// 
/// # Safety
/// `layout` must have non-zero size.
/// Attempting to allocate for a zero-sized `layout` may result in undefined behavior.
/// 
/// The returned block of memory may or may not be initialized.
pub unsafe fn alloc<A: ?Sized + Tier0Allocator>(alloc: &A, layout: Layout) -> *mut u8 {
	let align = layout.align();
	if align <= TIER0_MIN_ALIGN {
		return unsafe { check_aligned!("alloc"; align, alloc.alloc(layout.size())) }
	}

	let allocated_size = allocated_size(&layout);
	let result = unsafe { alloc.alloc(allocated_size) };
	if result.is_null() {
		return null_mut()
	}

	unsafe { aligned_ptr(result, layout) }
}

/// Shrinks or grows the block of memory pointed to by `ptr` to the given `new_size` in bytes.
/// 
/// The block is described by the given `ptr` and `layout`.
/// 
/// If this function returns null,
/// then ownership of the memory block has not been transferred to this allocator,
/// and the contents of the memory block are unaltered.
/// 
/// # Safety
/// The caller must ensure that:
/// - `ptr` is allocated via this allocator,
/// - `layout` is the same layout that was used to allocate that block of memory,
/// - `new_size` is greater than zero, and
/// - `new_size`, when rounded up to the nearest multiple of `layout.align()`,
///   does not overflow [`isize`]
///   (i.e. the rounded value must be less than or equal to [`isize::MAX`]).
pub unsafe fn realloc<A: ?Sized + Tier0Allocator>(alloc: &A, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
	let align = layout.align();
	if align <= TIER0_MIN_ALIGN {
		return unsafe { check_aligned!("realloc"; align, alloc.realloc(ptr, new_size)) }
	}

	let layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
	let allocated_size = allocated_size(&layout);
	let result = unsafe { alloc.realloc(unaligned_ptr(ptr, layout), allocated_size) };
	if result.is_null() {
		return null_mut()
	}
	
	unsafe { aligned_ptr(result, layout) }
}

/// Deallocates the block of memory at the given `ptr` with the given `layout`.
/// 
/// # Safety
/// The caller must ensure that:
/// - `ptr` is a block of memory currently allocated via this allocator, and
/// - `layout` is the same layout that was used to allocate that block of memory.
pub unsafe fn dealloc<A: ?Sized + Tier0Allocator>(alloc: &A, ptr: *mut u8, layout: Layout) {
	if layout.align() <= TIER0_MIN_ALIGN {
		return unsafe { alloc.free(ptr) }
	}

	unsafe {
		let ptr = unaligned_ptr(ptr, layout);
		alloc.free(ptr)
	}
}

/// Wrapper for [`Tier0Allocator`] unaligned allocators to implement [`GlobalAlloc`].
#[repr(transparent)]
pub struct Tier0GlobalAlloc<A: ?Sized + Tier0Allocator>(pub A);
unsafe impl<A: ?Sized + Tier0Allocator> GlobalAlloc for Tier0GlobalAlloc<A> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { alloc(&self.0, layout) }
	}
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		unsafe { realloc(&self.0, ptr, layout, new_size) }
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe { dealloc(&self.0, ptr, layout) }
	}
}

const fn allocated_size(layout: &Layout) -> usize {
	layout.align() + layout.size()
}

const fn aligned_ptr_offset(misalignment: usize, layout: &Layout) -> NonZeroUsize {
	if let Some(offset) = NonZeroUsize::new(misalignment) {
		offset
	} else {
		// Else, must fit a varint, but still be aligned.
		// SAFETY: Due to how Rust's type system works, `align() >= 1`.
		unsafe { NonZeroUsize::new_unchecked(layout.align()) }
	}
}

unsafe fn aligned_ptr(ptr: *mut u8, layout: Layout) -> *mut u8 {
	let alignment = layout.align();
	let misalignment = ptr.align_offset(alignment);
	let offset = aligned_ptr_offset(misalignment, &layout);
	unsafe {
		let result = ptr.byte_add(offset.get());
		varint_write_dec(result.byte_sub(1), misalignment);
		result
	}
}

unsafe fn unaligned_ptr(ptr: *mut u8, layout: Layout) -> *mut u8 {
	unsafe {
		let misalignment = varint_read_dec(ptr.byte_sub(1));
		let offset = aligned_ptr_offset(misalignment, &layout);
		ptr.byte_sub(offset.get())
	}
}

const VARINT_BITS: u32 = u8::BITS - 1;
const VARINT_CONT_BIT: u8 = 1 << VARINT_BITS;
const VARINT_MAX: u8 = VARINT_CONT_BIT - 1;

unsafe fn varint_write_dec(mut start: *mut u8, mut x: usize) {
	const VARINT_SHIFT_AMOUNT: u32 = usize::BITS - VARINT_BITS;
	const VARINT_MASK: usize = (VARINT_MAX as usize) << VARINT_SHIFT_AMOUNT;
	while x > VARINT_MAX as usize {
		unsafe {
			let piece = ((x & VARINT_MASK) >> VARINT_SHIFT_AMOUNT) as u8 | VARINT_CONT_BIT;
			*start = piece;
			x <<= VARINT_BITS;
			start = start.sub(1);
		}
	}
	unsafe { *start = x as u8 }
}

unsafe fn varint_read_dec(mut start: *mut u8) -> usize {
	let mut x = 0usize;
	loop {
		unsafe {
			let piece = *start;
			x <<= VARINT_BITS;
			if (piece & VARINT_CONT_BIT) != 0 {
				x |= piece as usize;
			} else {
				x |= (piece & VARINT_MAX) as usize;
				break x
			}
			start = start.sub(1);
		}
	}
}

use ::alloc::alloc::{
	GlobalAlloc, Layout,
};
use ::core::{
	num::NonZeroUsize,
	ptr::null_mut,
};

use crate::Tier0Allocator;

/// Wrapper for [`Tier0Allocator`] unaligned allocators to implement [`GlobalAlloc`].
#[repr(transparent)]
pub struct Tier0GlobalAlloc<A: ?Sized + Tier0Allocator>(pub A);
unsafe impl<A: ?Sized + Tier0Allocator> GlobalAlloc for Tier0GlobalAlloc<A> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		if layout.align() == 1 {
			unsafe { return self.0.alloc_unaligned(layout.size()) }
		}

		let allocated_size = allocated_size(&layout);
		let result = unsafe { self.0.alloc_unaligned(allocated_size) };
		if result.is_null() {
			return null_mut()
		}

		let result = unsafe { aligned_ptr(result, layout) };
		#[cfg(feature = "link-dll")]
		crate::con_warn!("alloc({:?}) -> {result:?}", layout.size());
		result
	}
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		if layout.align() == 1 {
			unsafe { return self.0.realloc_unaligned(ptr, new_size) }
		}

		let layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
		let allocated_size = allocated_size(&layout);
		let result = unsafe { self.0.realloc_unaligned(unaligned_ptr(ptr, layout), allocated_size) };
		if result.is_null() {
			return null_mut()
		}
		
		let result = unsafe { aligned_ptr(result, layout) };
		crate::con_warn!("realloc({ptr:?}, {result:?}, {new_size:?})");
		result
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		if layout.align() == 1 {
			unsafe { return self.0.free_unaligned(ptr) }
		}

		unsafe {
			crate::con_warn!("free({ptr:?})");
			let ptr = unaligned_ptr(ptr, layout);
			self.0.free_unaligned(ptr)
		}
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

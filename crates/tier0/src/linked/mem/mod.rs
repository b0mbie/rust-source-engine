use ::rse_cpp::{
	VtObject, virtual_call,
};

use crate::{
	Tier0Alloc,
	Tier0Allocator, Location,
};

use super::LinkedTier0;

pub mod cppdef;
use cppdef::*;

impl Tier0Alloc for LinkedTier0 {
	type Allocator<'a> = LinkedTier0Allocator where Self: 'a;
	fn allocator(&self) -> Self::Allocator<'_> {
		LinkedTier0Allocator
	}
}

macro_rules! with_alloc {
	($($arg:tt)*) => {
		virtual_call!(mem_alloc() => $($arg)*)
	};
}
fn mem_alloc() -> &'static VtObject<MemAllocVt> {
	unsafe { VtObject::from_ptr_const(g_pMemAlloc) }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0Allocator;

// SAFETY: The linked allocator uses `malloc` and friends, which are the Typical C/C++ Allocator.
unsafe impl Tier0Allocator for LinkedTier0Allocator {
	unsafe fn alloc(&self, size: usize) -> *mut u8 {
		unsafe { with_alloc!(alloc(size)) as _ }
	}
	unsafe fn realloc(&self, mem: *mut u8, new_size: usize) -> *mut u8 {
		unsafe { with_alloc!(realloc(mem as _, new_size)) as _ }
	}
	unsafe fn free(&self, mem: *mut u8) {
		unsafe { with_alloc!(free(mem as _)) }
	}
	unsafe fn debug_alloc(&self, size: usize, loc: Location<'_>) -> *mut u8 {
		unsafe { with_alloc!(debug_alloc(size, loc.filename.as_ptr(), loc.line)) as _ }
	}
	unsafe fn debug_realloc(&self, mem: *mut u8, new_size: usize, loc: Location<'_>) -> *mut u8 {
		unsafe { with_alloc!(debug_realloc(mem as _, new_size, loc.filename.as_ptr(), loc.line)) as _ }
	}
	unsafe fn debug_free(&self, mem: *mut u8, loc: Location<'_>) {
		unsafe { with_alloc!(debug_free(mem as _, loc.filename.as_ptr(), loc.line)) }
	}
	unsafe fn size_of(&self, mem: *mut u8) -> usize {
		unsafe { with_alloc!(get_size(mem as _)) }
	}
}

use ::core::ffi::{
	CStr, c_int, c_uint, c_void,
};

use crate::{
	Tier0Thread,
	ThreadId,
	ThreadFunc, RawThreadHandle,
};

use super::LinkedTier0;

pub mod cppdef;
use cppdef::*;

impl Tier0Thread for LinkedTier0 {
	unsafe fn create_simple_thread(
		&self, func: ThreadFunc, data: *mut c_void,
		stack_size: c_uint,
	) -> Option<RawThreadHandle> {
		unsafe { CreateSimpleThread(func, data, stack_size) }
	}
	fn release_thread_handle(&self, handle: Option<RawThreadHandle>) -> bool {
		unsafe { ReleaseThreadHandle(handle) }
	}

	fn sleep(&self, duration_millis: c_uint) {
		unsafe { ThreadSleep(duration_millis) }
	}

	fn get_current_handle(&self) -> RawThreadHandle {
		unsafe { ThreadGetCurrentHandle() }
	}

	fn get_priority(&self, handle: Option<RawThreadHandle>) -> c_int {
		unsafe { ThreadGetPriority(handle) }
	}
	fn set_priority(&self, handle: Option<RawThreadHandle>, priority: c_int) -> bool {
		unsafe { ThreadSetPriority(handle, priority) }
	}

	fn in_main_thread(&self) -> bool {
		unsafe { ThreadInMainThread() }
	}
	fn declare_current_is_main_thread(&self) {
		unsafe { DeclareCurrentThreadIsMainThread(); }
	}

	fn join(&self, handle: RawThreadHandle, timeout: c_uint) -> bool {
		unsafe { ThreadJoin(handle, timeout) }
	}
	unsafe fn detach(&self, handle: RawThreadHandle) {
		unsafe { ThreadDetach(handle) }
	}

	fn set_debug_name(&self, id: ThreadId, name: &CStr) {
		unsafe { ThreadSetDebugName(id, name.as_ptr()) }
	}
	fn set_affinity(&self, handle: Option<RawThreadHandle>, affinity_mask: c_int) {
		unsafe { ThreadSetAffinity(handle, affinity_mask) }
	}
}

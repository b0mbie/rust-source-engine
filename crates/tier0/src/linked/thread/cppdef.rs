use ::core::ffi::{
	c_char, c_uint, c_int, c_void,
};

use crate::{
	RawThreadHandle,
	ThreadFunc, ThreadId,
};

unsafe extern "C" {
	pub fn CreateSimpleThread(func: ThreadFunc, param: *mut c_void, stack_size: c_uint) -> Option<RawThreadHandle>;
	pub fn ReleaseThreadHandle(handle: Option<RawThreadHandle>) -> bool;

	pub fn ThreadSleep(duration: c_uint);
	// This function is only compiled in on 32-bit Windows.
	// pub fn ThreadGetCurrentId() -> ThreadId;
	pub fn ThreadGetCurrentHandle() -> RawThreadHandle;
	pub fn ThreadGetPriority(handle: Option<RawThreadHandle>) -> c_int;
	pub fn ThreadSetPriority(handle: Option<RawThreadHandle>, priority: c_int) -> bool;
	pub fn ThreadInMainThread() -> bool;
	pub fn DeclareCurrentThreadIsMainThread();

	pub fn ThreadJoin(handle: RawThreadHandle, timeout: c_uint) -> bool;
	pub fn ThreadDetach(handle: RawThreadHandle);

	pub fn ThreadSetDebugName(id: ThreadId, name: *const c_char);
	pub fn ThreadSetAffinity(handle: Option<RawThreadHandle>, affinity_mask: c_int);
}

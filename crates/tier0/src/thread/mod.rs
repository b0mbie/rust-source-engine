use ::core::{
	ffi::{
		CStr, c_int, c_uint, c_void,
	},
	ptr::NonNull,
};

#[cfg(target_pointer_width = "64")]
mod bitness_dependent {
	pub type ThreadId = u64;
}
#[cfg(not(target_pointer_width = "64"))]
mod bitness_dependent {
	pub type ThreadId = u32;
}

/// Type for thread identifiers.
pub use bitness_dependent::ThreadId;

mod data;
pub use data::*;
mod raw;
pub use raw::*;

pub type ThreadFunc = unsafe extern "C" fn(data: *mut c_void) -> crate::uintp;

pub trait Tier0Thread {
	/// # Safety
	/// `data` must be safe to use along with the given `func`.
	unsafe fn create_simple_thread(
		&self,
		func: ThreadFunc, data: *mut c_void,
		stack_size: c_uint,
	) -> Option<RawThreadHandle>;
	fn release_thread_handle(&self, handle: Option<RawThreadHandle>) -> bool;

	fn sleep(&self, duration_millis: c_uint);

	fn get_current_handle(&self) -> RawThreadHandle;

	fn get_priority(&self, handle: Option<RawThreadHandle>) -> c_int;
	fn set_priority(&self, handle: Option<RawThreadHandle>, priority: c_int) -> bool;

	fn in_main_thread(&self) -> bool;
	fn declare_current_is_main_thread(&self);

	fn join(&self, handle: RawThreadHandle, timeout: c_uint) -> bool;
	/// # Safety
	/// The thread function that was used for the initial creation of the thread
	/// must be able to execute soundly after the thread handle has been consumed.
	unsafe fn detach(&self, handle: RawThreadHandle);

	fn set_debug_name(&self, id: ThreadId, name: &CStr);
	fn set_affinity(&self, handle: Option<RawThreadHandle>, affinity_mask: c_int);
}

pub const TT_INFINITE: c_uint = 0xffffffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RawThreadHandle {
	raw: NonNull<ThreadHandleInner>,
}

impl RawThreadHandle {
	/// # Safety
	/// `raw` must be a handle returned from [`Tier0Thread`].
	pub const unsafe fn from_raw(raw: NonNull<ThreadHandleInner>) -> Self {
		Self {
			raw,
		}
	}

	pub const fn to_raw(&self) -> NonNull<ThreadHandleInner> {
		self.raw
	}
}

::rse_cpp::opaque_type! {
	pub struct ThreadHandleInner;
}

use ::core::{
	ffi::{
		c_uint, c_void,
	},
	mem::transmute,
};

use crate::{
	uintp,
	TT_INFINITE,
	Tier0Thread,
	RawThreadHandle,
	ThreadFunc,
};

use super::ThreadData;

pub struct RawThread<Tier0>
where
	Tier0: Tier0Thread,
{
	handle: RawThreadHandle,
	tier0: Tier0,
}

impl<Tier0> Drop for RawThread<Tier0>
where
	Tier0: Tier0Thread,
{
	fn drop(&mut self) {
		self.tier0.release_thread_handle(Some(self.handle));
	}
}

impl<Tier0> RawThread<Tier0>
where
	Tier0: Tier0Thread,
{
	/// # Safety
	/// `data` must be safe to use along with the given `func`.
	pub unsafe fn spawn_raw(
		tier0: Tier0,
		func: ThreadFunc, data: *mut c_void,
		stack_size: c_uint,
	) -> Option<Self> {
		let handle = unsafe { tier0.create_simple_thread(func, data, stack_size)? };
		Some(Self {
			handle,
			tier0,
		})
	}

	pub fn spawn<D>(
		tier0: Tier0,
		func: RawThreadFn<D>, data: ThreadData<D>,
		stack_size: usize,
	) -> Option<Self> {
		let func = unsafe { transmute::<RawThreadFn<D>, ThreadFunc>(func) };
		let stack_size = if stack_size > (c_uint::MAX as _) {
			c_uint::MAX
		} else {
			stack_size as _
		};
		unsafe { Self::spawn_raw(tier0, func, data.into_ptr(), stack_size) }
	}

	/// # Safety
	/// The thread function that was used for the initial creation of the thread
	/// must be able to execute soundly after the thread handle has been consumed.
	pub unsafe fn detach(self) {
		unsafe { self.tier0.detach(self.handle) }
	}

	pub fn join(self) -> bool {
		self.tier0.join(self.handle, TT_INFINITE)
	}

	pub fn join_with_timeout(self, timeout: c_uint) -> bool {
		self.tier0.join(self.handle, timeout)
	}
}

pub type RawThreadFn<D> = extern "C" fn(data: ThreadData<D>) -> uintp;

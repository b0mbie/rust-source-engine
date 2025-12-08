use crate::thread::MainThreadBound;

use super::exclusive::UnsafeExclusive;

#[repr(transparent)]
pub(crate) struct GlobalInterface<T> {
	bound: MainThreadBound<UnsafeExclusive<T>>,
}

impl<T> GlobalInterface<T> {
	pub const fn new(t: T) -> Self {
		Self {
			bound: MainThreadBound::new(UnsafeExclusive::new(t)),
		}
	}

	/// # Safety
	/// This function must not be called
	/// in a function that is called by
	/// any of the `inspect*` functions.
	pub unsafe fn inspect<R, F>(&self, f: F) -> R
	where
		F: FnOnce(Option<&mut T>) -> R,
	{
		if let Some(cell) = self.bound.get() {
			unsafe { cell.inspect(move |t| f(Some(t))) }
		} else {
			f(None)
		}
	}

	/// # Safety
	/// This function must not be called
	/// in a function that is called by
	/// any of the `inspect*` functions.
	/// 
	/// Additionally, this function must be called on the main thread.
	pub unsafe fn inspect_unchecked<R, F>(&self, f: F) -> R
	where
		F: FnOnce(&mut T) -> R,
	{
		let cell = unsafe { self.bound.get_unchecked() };
		unsafe { cell.inspect(f) }
	}

	/// # Safety
	/// This function must not be called
	/// in a function that is called by
	/// any of the `inspect*` functions.
	/// 
	/// Additionally, the operations performed on `T` must supporting multi-threading.
	pub unsafe fn inspect_mt<R, F>(&self, f: F) -> R
	where
		F: FnOnce(&T) -> R,
	{
		let cell = unsafe { self.bound.get_unchecked() };
		unsafe { cell.inspect(move |t| f(t)) }
	}
}

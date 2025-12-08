use ::core::cell::UnsafeCell;

/// [`Sync`] container that allows `unsafe` exclusive access to the inner `T`.
#[repr(transparent)]
pub struct UnsafeExclusive<T: ?Sized> {
	cell: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Sync for UnsafeExclusive<T> {}

impl<T: ?Sized> UnsafeExclusive<T> {
	pub const fn new(t: T) -> Self
	where
		T: Sized,
	{
		Self {
			cell: UnsafeCell::new(t),
		}
	}

	/// Provides unchecked exclusive access to the inner `T`.
	/// 
	/// # Safety
	/// This function must not be called
	/// in a function that is called by
	/// [`inspect`](Self::inspect).
	pub unsafe fn inspect<R, F>(&self, f: F) -> R
	where
		F: FnOnce(&mut T) -> R,
	{
		unsafe { f(&mut *self.cell.get()) }
	}
}

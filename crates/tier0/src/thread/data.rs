use ::core::{
	ffi::c_void,
	marker::PhantomData,
};

#[repr(transparent)]
pub struct ThreadData<T> {
	ptr: *mut c_void,
	_t: PhantomData<fn() -> T>,
}

impl<T> ThreadData<T> {
	/// # Safety
	/// `ptr` must valid to use with [`T::from_ptr`](ConvertThreadData::from_ptr).
	pub const unsafe fn from_ptr(ptr: *mut c_void) -> Self {
		Self {
			ptr,
			_t: PhantomData,
		}
	}

	pub const fn into_ptr(self) -> *mut c_void {
		self.ptr
	}

	pub fn new(t: T) -> Self
	where
		T: ConvertThreadData,
	{
		unsafe { Self::from_ptr(t.into_ptr()) }
	}

	pub fn get(self) -> T
	where
		T: ConvertThreadData,
	{
		unsafe { T::from_ptr(self.ptr) }
	}
}

pub trait ConvertThreadData {
	/// Consumes `self`, converting it to a pointer
	/// that can be used with [`from_ptr`](ConvertThreadData::from_ptr).
	fn into_ptr(self) -> *mut c_void;
	/// Converts a pointer to `Self`.
	/// 
	/// # Safety
	/// `ptr` must be a value previously returned by [`into_ptr`](ConvertThreadData::into_ptr)
	/// of the implementing type.
	unsafe fn from_ptr(ptr: *mut c_void) -> Self;
}
impl<T> ConvertThreadData for *mut T {
	fn into_ptr(self) -> *mut c_void {
		self as _
	}
	unsafe fn from_ptr(ptr: *mut c_void) -> Self {
		ptr as _
	}
}
impl<T> ConvertThreadData for *const T {
	fn into_ptr(self) -> *mut c_void {
		self as _
	}
	unsafe fn from_ptr(ptr: *mut c_void) -> Self {
		ptr as _
	}
}
impl ConvertThreadData for usize {
	fn into_ptr(self) -> *mut c_void {
		self as _
	}
	unsafe fn from_ptr(ptr: *mut c_void) -> Self {
		ptr as _
	}
}

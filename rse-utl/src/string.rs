use ::core::{
	alloc::Layout,
	cmp::Ordering,
	ffi::{
		CStr, c_char,
	},
	fmt::{
		self, Write,
	},
	mem::forget,
	ptr::null_mut,
	slice::from_raw_parts_mut,
};
use ::rse_cpp::c_str::opt_c_str_from_ptr;
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	Tier0Allocator,
};

use crate::cppdef::UtlString;

/// Transparent wrapper for `CUtlString`.
/// Not to be confused with Rust's [`CString`](::alloc::ffi::CString) type.
/// 
/// This is a buffer that stores a mutable C string inside of it
/// allocated on the heap with the [`LinkedTier0Allocator`].
/// It emulates the C++ `CUtlString` type for interacting with C++ code.
/// 
/// # Layout
/// This type has the exact same layout and ABI as [`UtlString`].
#[repr(transparent)]
pub struct CString(UtlString);

// SAFETY: `CString` always contains a *unique* pointer to a C string.
unsafe impl Send for CString {}
unsafe impl Sync for CString {}

impl CString {
	/// Returns an empty [`CString`].
	pub const fn new() -> Self {
		Self(UtlString {
			string: null_mut(),
		})
	}

	/// Returns a [`CString`].
	/// 
	/// # Safety
	/// [`inner.string`](UtlString::string) must point to
	/// a valid, mutable C string
	/// allocated with the [`LinkedTier0Allocator`].
	pub const unsafe fn from_inner(inner: UtlString) -> Self {
		Self(inner)
	}

	/// Consumes a [`CString`], returning the inner [`UtlString`].
	pub const fn into_inner(self) -> UtlString {
		let inner = UtlString {
			string: self.0.string,
		};
		forget(self);
		inner
	}

	/// Returns a mutable reference to a [`CString`] given a reference to the inner type.
	/// 
	/// # Safety
	/// [`inner.string`](UtlString::string) must point to
	/// a valid, mutable C string
	/// allocated with the [`LinkedTier0Allocator`].
	pub const unsafe fn from_mut(inner: &mut UtlString) -> &mut Self {
		unsafe { &mut *(inner as *mut UtlString as *mut Self) }
	}

	/// Returns an immutable reference to a [`CString`] given a reference to the inner type.
	/// 
	/// # Safety
	/// [`inner.string`](UtlString::string) must point to
	/// a valid, immutable C string
	/// allocated with the [`LinkedTier0Allocator`].
	pub const unsafe fn from_ref(inner: &UtlString) -> &Self {
		unsafe { &*(inner as *const UtlString as *const Self) }
	}

	/// Returns a mutable reference to a [`CString`] given a raw pointer.
	/// 
	/// See also [`from_mut`](Self::from_mut).
	/// 
	/// # Safety
	/// `ptr` must point to a valid, mutable [`UtlString`].
	/// Moreover,
	/// [`(*ptr).string`](UtlString::string) must point to
	/// a valid, mutable C string
	/// allocated with the [`LinkedTier0Allocator`].
	pub const unsafe fn from_mut_ptr<'a>(ptr: *mut UtlString) -> &'a mut Self {
		unsafe { &mut *(ptr as *mut Self) }
	}

	/// Returns an immutable reference to a [`CString`] given a raw pointer.
	/// 
	/// See also [`from_ref`](Self::from_ref).
	/// 
	/// # Safety
	/// `ptr` must point to a valid, immutable [`UtlString`].
	/// Moreover,
	/// [`(*ptr).string`](UtlString::string) must point to
	/// a valid, immutable C string
	/// allocated with the [`LinkedTier0Allocator`].
	pub const unsafe fn from_ptr<'a>(ptr: *const UtlString) -> &'a Self {
		unsafe { &*(ptr as *const Self) }
	}

	::rse_cpp::transparent_as_ptr_impls!(CString for UtlString as "UtlString");
	::rse_cpp::transparent_inner_impls!(CString for UtlString as "UtlString");

	/// Returns the [`CStr`] stored inside of this value,
	/// returning a static, immutable string for empty strings.
	pub const fn as_c_str(&self) -> &CStr {
		let ptr = self.0.string;
		if !ptr.is_null() {
			unsafe { CStr::from_ptr(ptr) }
		} else {
			c""
		}
	}

	/// Returns the [`CStr`] allocated inside of this value,
	/// or `None` if no string has been allocated.
	pub const fn as_alloc_c_str(&self) -> Option<&CStr> {
		unsafe { opt_c_str_from_ptr(self.0.string) }
	}

	/// Clears the string, making it empty.
	#[doc(alias = "Purge")]
	pub fn clear(&mut self) {
		let ptr = self.0.string;
		if !ptr.is_null() {
			unsafe { LinkedTier0Allocator.free(self.0.string as _) };
			self.0.string = null_mut();
		}
	}

	/// Returns the length of the C string in *bytes*.
	pub const fn len(&self) -> usize {
		match self.as_alloc_c_str() {
			Some(s) => s.count_bytes(),
			None => 0,
		}
	}

	/// Returns `true` if the C string is empty.
	pub const fn is_empty(&self) -> bool {
		if let Some(first) = unsafe { self.0.string.as_ref() } {
			*first == 0
		} else {
			true
		}
	}

	/// Converts this C string to its ASCII lower case equivalent in-place.
	pub fn make_ascii_lowercase(&mut self) {
		if let Some(bytes) = self.as_mut_bytes() {
			bytes.make_ascii_lowercase();
		}
	}

	/// Converts this C string to its ASCII upper case equivalent in-place.
	pub fn make_ascii_uppercase(&mut self) {
		if let Some(bytes) = self.as_mut_bytes() {
			bytes.make_ascii_uppercase();
		}
	}

	/// Returns a mutable slice of bytes stored inside of this value for mutation.
	pub const fn as_mut_bytes(&mut self) -> Option<&mut [u8]> {
		let ptr = self.0.string;
		if ptr.is_null() {
			return None
		}

		let c_str = unsafe { CStr::from_ptr(ptr) };
		let len = c_str.count_bytes();
		unsafe { Some(from_raw_parts_mut(ptr as _, len)) }
	}

	/// Copies the string `value` into the buffer.
	pub fn set(&mut self, value: &CStr) {
		let bytes = value.to_bytes();
		if !bytes.is_empty() {
			unsafe { self.set_unchecked(bytes) }
		} else {
			self.clear()
		}
	}

	/// Copies the slice of bytes into the buffer,
	/// truncating until the first NUL (`0x00`) character.
	pub fn set_bytes(&mut self, bytes: &[u8]) {
		if !bytes.is_empty() {
			if let Some(nul) = bytes.iter().position(move |&b| b == 0) {
				let until_nul = unsafe { bytes.get_unchecked(..nul) };
				let inner = unsafe { self.alloc_to(until_nul.len()) };
				inner.copy_from_slice(until_nul);
			} else {
				unsafe { self.set_unchecked(bytes) }
			}
		} else {
			self.clear()
		}
	}

	/// Copies the non-empty slice of bytes into the buffer.
	/// 
	/// # Safety
	/// `bytes` must not be empty,
	/// and must not contain inner NUL (`0x00`) characters.
	pub unsafe fn set_unchecked(&mut self, bytes: &[u8]) {
		let len = bytes.len();
		debug_assert_ne!(len, 0, "`set_non_empty` called with empty slice");
		let inner = unsafe { self.alloc_to(len) };
		inner.copy_from_slice(bytes);
	}

	/// Reallocates the internal buffer to accomodate for a byte slice of `length`,
	/// returning a mutable view of the buffer's string contents.
	/// 
	/// # Safety
	/// The returned slice of bytes may or may not be initialized.
	/// It must be initialized by the caller.
	pub unsafe fn alloc_to(&mut self, length: usize) -> &mut [u8] {
		let alloc_length = length + 1;
		let block = if !self.0.string.is_null() {
			unsafe { LinkedTier0Allocator.realloc(self.0.string as _, alloc_length) }
		} else {
			unsafe { LinkedTier0Allocator.alloc(alloc_length) }
		};

		if block.is_null() {
			::alloc::alloc::handle_alloc_error(unsafe {
				Layout::from_size_align_unchecked(alloc_length, 1)
			});
		}

		self.0.string = block as _;
		unsafe { *self.0.string.wrapping_add(length) = 0 };

		unsafe { from_raw_parts_mut(self.0.string as *mut u8, length) }
	}

	/// Consumes this C string buffer,
	/// transferring ownership of the C string.
	/// 
	/// The returned pointer must be used with [`from_raw`](Self::from_raw) to deallocate the [`CString`] in Rust code;
	/// however, it is permitted to pass the pointer to external code
	/// if it itself can deallocate it using [`LinkedTier0Allocator`].
	pub const fn into_raw(self) -> *mut c_char {
		let ptr = self.0.string;
		forget(self);
		ptr
	}

	/// Takes ownership of a [`Self`] that is represented by a pointer.
	/// 
	/// # Safety
	/// This function must only be called with either
	/// a pointer returned by [`into_raw`](Self::into_raw), or
	/// a valid pointer to a C string that was allocated with [`LinkedTier0Allocator`]
	/// otherwise.
	pub const unsafe fn from_raw(ptr: *mut c_char) -> Self {
		Self(UtlString {
			string: ptr,
		})
	}
}

impl Drop for CString {
	fn drop(&mut self) {
		self.clear();
	}
}

impl Default for CString {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Debug for CString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}

impl fmt::Display for CString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let bytes = self.as_c_str().to_bytes();
		for chunk in bytes.utf8_chunks() {
			f.write_str(chunk.valid())?;
			if !chunk.invalid().is_empty() {
				f.write_char(char::REPLACEMENT_CHARACTER)?;
			}
		}
		Ok(())
	}
}

impl PartialEq for CString {
	fn eq(&self, other: &Self) -> bool {
		self.as_alloc_c_str() == other.as_alloc_c_str()
	}
}
impl Eq for CString {}

impl Ord for CString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_c_str().cmp(other.as_c_str())
	}
}
impl PartialOrd for CString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<::alloc::string::String> for CString {
	fn from(value: ::alloc::string::String) -> Self {
		let mut s = Self::new();
		s.set_bytes(value.as_bytes());
		s
	}
}

impl From<&str> for CString {
	fn from(value: &str) -> Self {
		let mut s = Self::new();
		s.set_bytes(value.as_bytes());
		s
	}
}

impl From<&[u8]> for CString {
	fn from(value: &[u8]) -> Self {
		let mut s = Self::new();
		s.set_bytes(value);
		s
	}
}

impl From<&CStr> for CString {
	fn from(value: &CStr) -> Self {
		let mut s = Self::new();
		s.set(value);
		s
	}
}

impl AsRef<CStr> for CString {
	fn as_ref(&self) -> &CStr {
		self.as_c_str()
	}
}

impl AsRef<[u8]> for CString {
	fn as_ref(&self) -> &[u8] {
		self.as_c_str().to_bytes()
	}
}

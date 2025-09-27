extern crate alloc;

use ::core::{
	alloc::Layout,
	cmp::Ordering,
	ffi::CStr,
	fmt::{
		self, Write,
	},
	ptr::null_mut,
	slice::from_raw_parts_mut,
};
use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	Tier0Allocator,
};

use crate::cppdef::UtlString;

::rse_cpp::transparent_wrapper! {
	/// Transparent wrapper for `CUtlString`.
	/// Not to be confused with Rust's `alloc::ffi::CString` type.
	/// 
	/// This is a buffer that stores a mutable C string inside of it
	/// allocated on the heap with the global `tier0` allocator.
	/// It emulates the C++ `CUtlString` type for interacting with C++ code.
	/// 
	/// # Layout
	/// This type has the exact same layout and ABI as [`UtlString`].
	pub struct CString for UtlString as "UtlString";
}

impl CString {
	/// Returns an empty [`CString`].
	pub const fn new() -> Self {
		Self(UtlString {
			string: null_mut(),
		})
	}

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

	/// Returns the non-empty [`CStr`] stored inside of this value,
	/// or `None` if the string is empty.
	pub const fn as_non_empty_c_str(&self) -> Option<&CStr> {
		let ptr = self.0.string;
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}

	/// Clears the string, making it empty.
	#[doc(alias = "purge")]
	pub fn clear(&mut self) {
		let ptr = self.0.string;
		if !ptr.is_null() {
			unsafe { LinkedTier0Allocator.free(self.0.string as _) };
			self.0.string = null_mut();
		}
	}

	/// Returns the length of the C string in *bytes*.
	pub const fn len(&self) -> usize {
		match self.as_non_empty_c_str() {
			Some(s) => s.count_bytes(),
			None => 0,
		}
	}

	/// Returns `true` if the C string is empty.
	pub const fn is_empty(&self) -> bool {
		self.0.string.is_null()
	}

	/// Converts this C string to its ASCII lower case equivalent in-place.
	pub fn make_ascii_lowercase(&mut self) {
		if let Some(bytes) = unsafe { self.as_mut_bytes() } {
			bytes.make_ascii_lowercase();
		}
	}

	/// Converts this C string to its ASCII upper case equivalent in-place.
	pub fn make_ascii_uppercase(&mut self) {
		if let Some(bytes) = unsafe { self.as_mut_bytes() } {
			bytes.make_ascii_uppercase();
		}
	}

	/// Returns a mutable slice of bytes stored inside of this value for mutation.
	/// 
	/// # Safety
	/// The returned slice should not have any NUL (`0x00`) bytes inserted into it.
	pub const unsafe fn as_mut_bytes(&mut self) -> Option<&mut [u8]> {
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
			unsafe { self.set_non_empty(bytes) }
		} else {
			self.clear()
		}
	}

	/// Copies the non-empty slice of bytes into the buffer.
	/// 
	/// # Safety
	/// `bytes` must not be empty,
	/// and must not contain inner NUL (`0x00`) characters.
	pub unsafe fn set_non_empty(&mut self, bytes: &[u8]) {
		let len = bytes.len();
		debug_assert_ne!(len, 0, "`set_non_empty` called with empty slice");
		let inner = self.alloc_bytes(len);
		inner.copy_from_slice(bytes);
	}

	fn alloc_bytes(&mut self, length: usize) -> &mut [u8] {
		let alloc_length = length + 1;
		let block = if !self.0.string.is_null() {
			unsafe { LinkedTier0Allocator.realloc(self.0.string as _, alloc_length) }
		} else {
			unsafe { LinkedTier0Allocator.alloc(alloc_length) }
		};

		if block.is_null() {
			alloc::alloc::handle_alloc_error(unsafe {
				Layout::from_size_align_unchecked(alloc_length, 1)
			});
		}

		self.0.string = block as _;
		unsafe { *self.0.string.wrapping_add(length) = 0 };

		unsafe { from_raw_parts_mut(self.0.string as *mut u8, length) }
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
		self.as_non_empty_c_str() == other.as_non_empty_c_str()
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

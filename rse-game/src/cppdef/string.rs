use ::core::{
	cmp::Ordering,
	ffi::{
		CStr, c_char,
	},
	fmt,
	hash::Hash,
	ptr::null,
};

#[derive(Default)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct string_t {
	value: *const c_char,
}

impl string_t {
	pub const NULL: Self = Self {
		value: null(),
	};

	pub const fn new(s: &'static CStr) -> Self {
		Self {
			value: s.as_ptr(),
		}
	}

	/// # Safety
	/// `ptr` *must* either be null, or point to a valid C string.
	pub const unsafe fn from_ptr(ptr: *const c_char) -> Self {
		Self {
			value: ptr,
		}
	}

	pub const fn as_ptr(&self) -> *const c_char {
		self.value
	}

	pub const fn to_c_str(&self) -> &CStr {
		if self.value.is_null() {
			c""
		} else {
			unsafe { CStr::from_ptr(self.value) }
		}
	}
}

impl fmt::Debug for string_t {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.to_c_str().fmt(f)
	}
}

impl Ord for string_t {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_c_str().cmp(other.to_c_str())
	}
}
impl PartialOrd for string_t {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl Eq for string_t {}
impl PartialEq for string_t {
	fn eq(&self, other: &Self) -> bool {
		self.to_c_str().eq(other.to_c_str())
	}
}

impl Hash for string_t {
	fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
		self.to_c_str().hash(state)
	}
}

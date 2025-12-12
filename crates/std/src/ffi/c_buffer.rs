use ::core::{
	cmp::Ordering,
	ffi::{
		CStr, c_char, c_float,
	},
	fmt::{
		Write, self,
	},
	hash::{
		Hash, Hasher,
	},
};

use crate::fmt_util::CStrWrite;

use super::CString;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CBuffer<const N: usize> {
	bytes: [u8; N],
}

impl<const N: usize> CBuffer<N> {
	pub const fn new() -> Self {
		Self {
			bytes: [0; _],
		}
	}

	pub const fn capacity(&self) -> usize {
		self.bytes.len()
	}

	pub const fn as_ptr(&self) -> *const c_char {
		self.bytes.as_ptr() as _
	}

	pub const fn as_mut_ptr(&mut self) -> *mut c_char {
		self.bytes.as_mut_ptr() as _
	}

	pub const fn bytes(&self) -> &[u8; N] {
		&self.bytes
	}

	/// # Safety
	/// The returned buffer must be terminated with a NUL character
	/// by the time the [`CBuffer`] is available again.
	pub const unsafe fn bytes_mut(&mut self) -> &mut [u8; N] {
		&mut self.bytes
	}

	pub const fn as_c_str(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.as_ptr()) }
	}

	pub fn write<F>(&mut self, f: F)
	where
		F: FnOnce(&mut CBufferWrite<'_>),
	{
		unsafe {
			let Some(inner) = CStrWrite::new(self.bytes_mut()) else {
				return
			};
			let mut writer = CBufferWrite { inner, };
			f(&mut writer);
			writer.inner.finish();
		}
	}

	pub(crate) fn print_float(&mut self, value: c_float) {
		self.write(move |f| {
			let _ = write!(f, "{value}");
		})
	}
}

impl<const N: usize> Default for CBuffer<N> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const N: usize> fmt::Debug for CBuffer<N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}

impl<const N: usize> PartialEq for CBuffer<N> {
	fn eq(&self, other: &Self) -> bool {
		self.as_c_str() == other.as_c_str()
	}
}
impl<const N: usize> Eq for CBuffer<N> {}

impl<const N: usize> PartialOrd for CBuffer<N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl<const N: usize> Ord for CBuffer<N> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_c_str().cmp(other.as_c_str())
	}
}

impl<const N: usize> Hash for CBuffer<N> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_c_str().hash(state)
	}
}

impl<const N: usize> From<CBuffer<N>> for CString {
	fn from(value: CBuffer<N>) -> Self {
		Self::from(value.as_c_str())
	}
}

#[repr(transparent)]
pub struct CBufferWrite<'a> {
	inner: CStrWrite<'a>,
}

impl<'a> CBufferWrite<'a> {
	pub fn write(&mut self, data: &[u8]) -> usize {
		self.inner.write(data)
	}
}

impl Write for CBufferWrite<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes());
		Ok(())
	}
}

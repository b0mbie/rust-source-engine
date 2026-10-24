use ::core::{
	ffi::CStr,
	marker::PhantomData,
	ops::{
		Deref, DerefMut,
	},
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};
use ::rse_cpp::{
	c_str::opt_c_str_as_ptr,
	util::clamp_len_to_c_int,
};

use crate::cppdef::{
	bf_read, bf_write,
};

use super::{
	BfRead, BfWrite,
};

const fn default_bit_count(n_bytes: usize) -> usize {
	match n_bytes.checked_shl(3) {
		Some(n) => n,
		_ => isize::MAX as _,
	}
}

#[repr(transparent)]
pub struct SliceBfRead<'a> {
	raw: bf_read,
	_slice: PhantomData<&'a [u8]>,
}

impl<'a> SliceBfRead<'a> {
	pub const fn new(data: &'a [u32]) -> Self {
		Self::with(data, default_bit_count(data.len()), 0, None)
	}

	pub const fn with(
		data: &'a [u32], n_bits: usize,
		start_bit: usize,
		debug_name: Option<&'a CStr>,
	) -> Self {
		unsafe {
			let bytes = from_raw_parts(data.as_ptr().cast::<u8>(), data.len());
			Self::aligned_with(bytes, n_bits, start_bit, debug_name)
		}
	}

	/// # Safety
	/// `aligned_bytes` must have an alignment of `4` - the same as [`u32`].
	pub const unsafe fn aligned(aligned_bytes: &'a [u8]) -> Self {
		unsafe { Self::aligned_with(aligned_bytes, default_bit_count(aligned_bytes.len()), 0, None) }
	}

	/// # Safety
	/// `aligned_bytes` must have an alignment of `4` - the same as [`u32`].
	pub const unsafe fn aligned_with(
		aligned_bytes: &'a [u8], n_bits: usize,
		start_bit: usize,
		debug_name: Option<&'a CStr>,
	) -> Self {
		let raw = bf_read {
			assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(debug_name),
			data: aligned_bytes.as_ptr(),
			data_bytes: clamp_len_to_c_int(aligned_bytes.len()),
			data_bits: clamp_len_to_c_int(n_bits),
			current_bit: clamp_len_to_c_int(start_bit),
			overflow: false,
		};
		Self {
			raw,
			_slice: PhantomData,
		}
	}

	pub const fn as_inner(&self) -> &BfRead {
		unsafe { BfRead::from_ref(&self.raw) }
	}

	pub const fn as_mut_inner(&mut self) -> &mut BfRead {
		unsafe { BfRead::from_mut(&mut self.raw) }
	}
}
impl Deref for SliceBfRead<'_> {
	type Target = BfRead;
	fn deref(&self) -> &Self::Target { self.as_inner() }
}
impl DerefMut for SliceBfRead<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_inner() }
}

#[repr(transparent)]
pub struct SliceBfWrite<'a> {
	raw: bf_write,
	_slice: PhantomData<&'a [u8]>,
}

impl<'a> SliceBfWrite<'a> {
	pub const fn new(data: &'a mut [u32]) -> Self {
		Self::with(data, default_bit_count(data.len()), 0, None)
	}

	pub const fn with(
		data: &'a mut [u32], n_bits: usize,
		start_bit: usize,
		debug_name: Option<&'a CStr>,
	) -> Self {
		unsafe {
			let bytes = from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), data.len());
			Self::aligned_with(bytes, n_bits, start_bit, debug_name)
		}
	}

	/// # Safety
	/// `aligned_bytes` must have an alignment of `4` - the same as [`u32`].
	pub const unsafe fn aligned(aligned_bytes: &'a mut [u8]) -> Self {
		unsafe { Self::aligned_with(aligned_bytes, default_bit_count(aligned_bytes.len()), 0, None) }
	}

	/// # Safety
	/// `aligned_bytes` must have an alignment of `4` - the same as [`u32`].
	pub const unsafe fn aligned_with(
		aligned_bytes: &'a mut [u8], n_bits: usize,
		start_bit: usize,
		debug_name: Option<&'a CStr>,
	) -> Self {
		let raw = bf_write {
			assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(debug_name),
			data: aligned_bytes.as_mut_ptr().cast(),
			data_bytes: clamp_len_to_c_int(aligned_bytes.len()),
			data_bits: clamp_len_to_c_int(n_bits),
			current_bit: clamp_len_to_c_int(start_bit),
			overflow: false,
		};
		Self {
			raw,
			_slice: PhantomData,
		}
	}

	pub const fn as_inner(&self) -> &BfWrite {
		unsafe { BfWrite::from_ref(&self.raw) }
	}

	pub const fn as_mut_inner(&mut self) -> &mut BfWrite {
		unsafe { BfWrite::from_mut(&mut self.raw) }
	}
}
impl Deref for SliceBfWrite<'_> {
	type Target = BfWrite;
	fn deref(&self) -> &Self::Target { self.as_inner() }
}
impl DerefMut for SliceBfWrite<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_inner() }
}

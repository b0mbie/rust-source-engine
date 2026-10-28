use ::core::{
	ffi::{
		CStr, c_int, c_uint,
	},
	fmt,
	ptr::{
		null, null_mut,
	},
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};
use ::rse_cpp::{
	c_str::opt_c_str_as_ptr,
	transparent_wrapper,
};

use crate::cppdef::{
	bf_read, bf_write,
};

mod borrowed;
pub use borrowed::*;

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout and ABI as [`bf_read`].
	pub struct BfRead for bf_read as "bf_read";
}

impl BfRead {
	pub const fn empty() -> Self {
		Self::named_empty(None)
	}

	pub const fn named_empty(name: Option<&'static CStr>) -> Self {
		Self(bf_read {
			data: null(),
			data_bytes: 0, data_bits: -1,
			current_bit: 0,
			overflow: false, assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(name),
		})
	}

	pub const fn reset(&mut self) {
		self.0.current_bit = 0;
		self.0.overflow = false;
	}

	pub const fn bytes(&self) -> &[u8] {
		let len = self.as_inner().data_bytes;
		let len = if len >= 0 {
			len as usize
		} else {
			0
		};
		unsafe { from_raw_parts(self.as_inner().data, len) }
	}
}

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout and ABI as [`bf_write`].
	pub struct BfWrite for bf_write as "bf_write";
}

impl BfWrite {
	pub const fn empty() -> Self {
		Self::named_empty(None)
	}

	pub const fn named_empty(name: Option<&'static CStr>) -> Self {
		Self(bf_write {
			data: null_mut(),
			data_bytes: 0, data_bits: -1,
			current_bit: 0,
			overflow: false, assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(name),
		})
	}

	pub const fn reset(&mut self) {
		self.0.current_bit = 0;
		self.0.overflow = false;
	}

	pub const fn as_bytes(&self) -> &[u8] {
		let len = self.as_inner().data_bytes;
		let len = if len >= 0 {
			len as usize
		} else {
			0
		};
		unsafe { from_raw_parts(self.as_inner().data.cast::<u8>(), len) }
	}

	pub const fn as_mut_bytes(&mut self) -> &mut [u8] {
		let len = self.as_inner().data_bytes;
		let len = if len >= 0 {
			len as usize
		} else {
			0
		};
		unsafe { from_raw_parts_mut(self.as_mut_inner().data.cast::<u8>(), len) }
	}

	pub const fn as_mut_dwords(&mut self) -> &mut [u32] {
		let len = self.as_inner().data_bytes;
		let len = if len >= 0 {
			len as usize
		} else {
			0
		};
		unsafe { from_raw_parts_mut(self.as_mut_inner().data, len / size_of::<u32>()) }
	}

	pub const fn bytes_left_mut(&mut self) -> Option<&mut [u8]> {
		let n_bytes = {
			let n_bytes = self.as_inner().data_bytes;
			if n_bytes >= 0 {
				n_bytes as usize
			} else {
				0
			}
		};
		let current_byte = {
			let current_bit = self.as_inner().current_bit;
			if current_bit >= 0 {
				(current_bit / (u8::BITS as c_int)) as usize
			} else {
				0
			}
		};
		match n_bytes.checked_sub(current_byte) {
			Some(len) => Some(unsafe {
				from_raw_parts_mut(
					self.as_mut_inner()
						.data.cast::<u8>()
						.add(current_byte),
					len,
				)
			}),
			_ => None,
		}
	}

	pub const fn advance_bits(&mut self, bits: c_uint) {
		unsafe {
			let inner = self.as_mut_inner();
			inner.current_bit = inner.current_bit.saturating_add_unsigned(bits)
		}
	}

	pub const fn write_bytes(&mut self, bytes: &[u8]) {
		if let Some(shift) = ByteWriteShr::new(self.as_inner().current_bit) {
			self.write_bytes_shifted(shift, bytes)
		} else {
			self.write_bytes_unshifted(bytes)
		}
	}

	pub const fn write_bits(&mut self, value: u8, bits_to_write: Bits) {
		let shift = ByteWriteShr::new(self.as_inner().current_bit);
		self.write_bits_impl(shift, value, bits_to_write)
	}

	const fn write_bytes_unshifted(&mut self, bytes: &[u8]) {
		let Some(bytes_left) = self.bytes_left_mut() else { return };
		let len = {
			let available = bytes_left.len();
			let want = bytes.len();
			clamp_len_to_c_uint(if want <= available {want} else {available})
		};
		let (bytes_left, bytes_to_write) = unsafe {
			(
				from_raw_parts_mut(bytes_left.as_mut_ptr(), len as _),
				from_raw_parts(bytes.as_ptr(), len as _),
			)
		};
		bytes_left.copy_from_slice(bytes_to_write);
		self.advance_bits(len.saturating_mul(u8::BITS as _))
	}

	const fn write_bytes_shifted(&mut self, right_shift: ByteWriteShr, bytes: &[u8]) {
		let Some(bytes_left) = self.bytes_left_mut() else { return };
		let len = {
			let available = bytes_left.len();
			let want = bytes.len();
			clamp_len_to_c_uint(if want <= available {want} else {available})
		};
		if len == 0 { return }
		let bytes_to_write = unsafe { from_raw_parts(bytes.as_ptr(), len as _) };

		let right_shift = right_shift as u8;
		let left_shift = (u8::BITS as u8) - right_shift;

		bytes_left[0] = bytes_to_write[0] >> right_shift;
		
		let mut i = 1usize;
		while i < (len as usize) {
			let prev = bytes_to_write[i - 1] << left_shift;
			let current = bytes_to_write[i] >> right_shift;
			bytes_left[i] = prev | current;
			i += 1;
		}

		self.advance_bits(len.saturating_mul(u8::BITS as _))
	}

	const fn write_bits_impl(
		&mut self, shift: Option<ByteWriteShr>,
		value: u8, bits_to_write: Bits,
	) {
		let Some([first, rest @ ..]) = self.bytes_left_mut() else { return };

		let bits_to_write = bits_to_write as u8;
		let value_shift = 8 - bits_to_write;
		let mask = u8::MAX >> value_shift;
		let left_aligned_value = (value & mask) << value_shift;

		let occupied_bits_in_first = match shift {
			None => 0,
			Some(s) => s as u8,
		};
		*first |= left_aligned_value >> occupied_bits_in_first;
		
		if let [second, ..] = rest
		&& let Some(left_shift) =
			(u8::BITS as u8 * 2).checked_sub(occupied_bits_in_first.saturating_add(bits_to_write))
		{
			*second = value.unbounded_shl(left_shift as _);
		}

		self.advance_bits(bits_to_write as _);
	}
}

impl fmt::Write for BfWrite {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_bytes(s.as_bytes());
		Ok(())
	}
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum ByteWriteShr {
	B1 = 1,
	B2 = 2,
	B3 = 3,
	B4 = 4,
	B5 = 5,
	B6 = 6,
	B7 = 7,
}

impl ByteWriteShr {
	pub const fn new(current_bit: c_int) -> Option<Self> {
		match current_bit % 8 {
			0 => None,
			1 => Some(Self::B1),
			2 => Some(Self::B2),
			3 => Some(Self::B3),
			4 => Some(Self::B4),
			5 => Some(Self::B5),
			6 => Some(Self::B6),
			7 => Some(Self::B7),
			_ => unreachable!(),
		}
	}
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Bits {
	B1 = 1,
	B2 = 2,
	B3 = 3,
	B4 = 4,
	B5 = 5,
	B6 = 6,
	B7 = 7,
	B8 = 8,
}

const fn clamp_len_to_c_uint(x: usize) -> c_uint {
	if x > (c_uint::MAX as usize) {c_uint::MAX} else {x as _}
}

#[cfg(test)]
mod tests {
	use super::{
		BfWrite, SliceBfWrite,
		Bits, ByteWriteShr,
	};

	fn with_buf(n_bits: usize, start_bit: usize, f: impl FnOnce(&mut BfWrite)) {
		let mut data = [0u32; 1];
		let mut buf = SliceBfWrite::with(&mut data, n_bits, start_bit, None);
		f(&mut buf)
	}

	#[test]
	fn bf_write_bits_impl() {
		with_buf(12, 0, |b| {
			b.write_bits_impl(None, u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b10000000, 0, 0, 0], "buffer bytes check failed");

			b.write_bits_impl(Some(ByteWriteShr::B1), u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b11000000, 0, 0, 0], "buffer bytes check failed");
		});

		with_buf(12, 0, |b| {
			b.write_bits_impl(Some(ByteWriteShr::B1), u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b01000000, 0, 0, 0], "buffer bytes check failed");
			b.write_bits_impl(None, u8::MAX, Bits::B3);
			assert_eq!(b.as_bytes(), &[0b11100000, 0, 0, 0], "buffer bytes check failed");
		});
	}

	#[test]
	fn bf_write_bits() {
		with_buf(12, 0, |b| {
			b.write_bits(u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b10000000, 0, 0, 0], "buffer bytes check failed");

			b.write_bits_impl(Some(ByteWriteShr::B1), u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b11000000, 0, 0, 0], "buffer bytes check failed");
		});

		with_buf(12, 0, |b| {
			b.write_bits_impl(Some(ByteWriteShr::B1), u8::MAX, Bits::B1);
			assert_eq!(b.as_bytes(), &[0b01000000, 0, 0, 0], "buffer bytes check failed");
			b.write_bits_impl(None, u8::MAX, Bits::B3);
			assert_eq!(b.as_bytes(), &[0b11100000, 0, 0, 0], "buffer bytes check failed");
		});
	}

	#[test]
	fn bf_write_bytes_impl() {
		with_buf(12, 0, |b| {
			b.write_bytes_unshifted(b"la");
			assert_eq!(b.as_bytes(), &[b'l', b'a', 0, 0], "buffer bytes check failed");
			b.write_bytes_unshifted(b"x");
			assert_eq!(b.as_bytes(), &[b'l', b'a', b'x', 0], "buffer bytes check failed");
		});

		with_buf(12, 0, |b| {
			b.write_bytes_shifted(ByteWriteShr::B1, b"la");
			assert_eq!(b.as_bytes(), &[b'l' >> 1, b'a' >> 1, 0, 0], "buffer bytes check failed");
			b.write_bytes_shifted(ByteWriteShr::B3, b"x");
			assert_eq!(b.as_bytes(), &[b'l' >> 1, b'a' >> 1, b'x' >> 3, 0], "buffer bytes check failed");
		});
	}
}

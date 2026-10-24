use ::core::{
	fmt::{
		self, Write as _,
	},
	iter::once,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

pub use ::libc::wchar_t;

#[repr(C)]
pub struct WCStrBuf<const LEN: usize> {
	wchars: [wchar_t; LEN],
	nul: wchar_t,
}

impl<const LEN: usize> Default for WCStrBuf<LEN> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const LEN: usize> WCStrBuf<LEN> {
	const fn check_len() {
		if LEN >= isize::MAX as _ {
			panic!("`LEN` cannot be `isize::MAX` or higher")
		}
	}

	pub const fn from_wchars(wchars: [wchar_t; LEN]) -> Self {
		Self::check_len();
		Self {
			wchars,
			nul: 0,
		}
	}

	pub const fn new() -> Self {
		Self::from_wchars([0; _])
	}

	pub fn write_chars<I: IntoIterator<Item = char>>(&mut self, chars: I) {
		let chars = chars.into_iter();
		for (wchar, char) in self.wchars.iter_mut().zip(chars.chain(once('\0'))) {
			*wchar = char as _;
		}
	}

	pub const fn as_ptr(&self) -> *const wchar_t {
		self as *const _ as *const wchar_t
	}

	pub const fn as_mut_ptr(&mut self) -> *mut wchar_t {
		self as *mut _ as *mut wchar_t
	}

	pub const fn as_wchars(&self) -> &[wchar_t] {
		&self.wchars
	}

	pub const unsafe fn as_mut_wchars(&mut self) -> &mut [wchar_t] {
		&mut self.wchars
	}

	pub const fn as_wchars_with_nul(&self) -> &[wchar_t] {
		unsafe { from_raw_parts(self.as_ptr(), LEN.saturating_add(1)) }
	}

	pub const unsafe fn as_mut_wchars_with_nul(&mut self) -> &mut [wchar_t] {
		unsafe { from_raw_parts_mut(self.as_mut_ptr(), LEN.saturating_add(1)) }
	}

	pub const fn to_wcstr(&self) -> &WCStr {
		unsafe { WCStr::from_wchars_with_nul_unchecked(self.as_wchars_with_nul()) }
	}

	pub const fn to_mut_wcstr(&mut self) -> &mut WCStr {
		unsafe { WCStr::from_mut_wchars_with_nul_unchecked(self.as_mut_wchars_with_nul()) }
	}

	pub const fn from_chars(chars: [char; LEN]) -> Self {
		let mut wchars: [wchar_t; LEN] = [0; _];
		let mut i = 0;
		while i < LEN {
			wchars[i] = chars[i] as _;
			i += 1;
		}
		Self::from_wchars(wchars)
	}

	pub const fn from_bytes(bytes: [u8; LEN]) -> Self {
		let mut wchars: [wchar_t; LEN] = [0; _];
		let mut i = 0;
		while i < LEN {
			wchars[i] = bytes[i] as _;
			i += 1;
		}
		Self::from_wchars(wchars)
	}
}

#[repr(transparent)]
pub struct WCStr {
	wchars_with_nul: [wchar_t],
}

impl WCStr {
	pub const unsafe fn from_wchars_with_nul_unchecked(slice: &[wchar_t]) -> &WCStr {
		unsafe { &*(slice as *const _ as *const Self) }
	}

	pub const unsafe fn from_mut_wchars_with_nul_unchecked(slice: &mut [wchar_t]) -> &mut WCStr {
		unsafe { &mut *(slice as *mut _ as *mut Self) }
	}

	pub unsafe fn from_ptr<'a>(ptr: *const wchar_t) -> &'a WCStr {
		unsafe {
			let len = wcslen(ptr);
			Self::from_wchars_with_nul_unchecked(from_raw_parts(ptr, len))
		}
	}

	pub unsafe fn from_mut_ptr<'a>(ptr: *mut wchar_t) -> &'a mut WCStr {
		unsafe {
			let len = wcslen(ptr);
			Self::from_mut_wchars_with_nul_unchecked(from_raw_parts_mut(ptr, len))
		}
	}

	pub const fn as_ptr(&self) -> *const wchar_t {
		self.wchars_with_nul.as_ptr()
	}

	pub const fn as_mut_ptr(&mut self) -> *mut wchar_t {
		self.wchars_with_nul.as_mut_ptr()
	}

	pub const fn len(&self) -> usize {
		self.wchars_with_nul.len()
	}

	pub const fn is_empty(&self) -> bool {
		self.wchars_with_nul.is_empty()
	}

	pub const fn to_wchars_with_nul(&self) -> &[wchar_t] {
		&self.wchars_with_nul
	}

	pub const unsafe fn to_mut_wchars_with_nul(&mut self) -> &mut [wchar_t] {
		&mut self.wchars_with_nul
	}

	pub const fn to_wchars(&self) -> &[wchar_t] {
		unsafe {
			let len_without_nul = self.len().unchecked_sub(1);
			from_raw_parts(self.wchars_with_nul.as_ptr(), len_without_nul)
		}
	}

	pub fn display(&self) -> Display<'_> {
		Display {
			wchars: self.to_wchars(),
		}
	}
}

impl fmt::Debug for WCStr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_char('"')?;
		for &wchar in self.to_wchars() {
			let codepoint = wchar as u32;
			if let Some(c) = char::from_u32(codepoint) {
				f.write_char(c)
			} else {
				f.write_str("\\u")?;
				f.write_fmt(format_args!("{codepoint:04x}"))
			}?;
		}
		f.write_char('"')
	}
}

pub struct Display<'a> {
	wchars: &'a [wchar_t],
}

impl fmt::Display for Display<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for &wchar in self.wchars {
			f.write_char(
				char::from_u32(wchar as u32)
					.unwrap_or(char::REPLACEMENT_CHARACTER)
			)?;
		}
		Ok(())
	}
}

// TODO: `const` without `libc`?
unsafe fn wcslen(ptr: *const wchar_t) -> usize {
	unsafe { ::libc::wcslen(ptr) }
}

#[test]
fn buf_works() {
	fn test_with_str<const LEN: usize>(input: &str) {
		let mut buf = WCStrBuf::<LEN>::new();
		buf.write_chars(input.chars());

		let chars = buf.as_wchars().iter()
			.map(move |&c| char::from_u32(c as _))
			;

		for (i, (actual, expected)) in chars.zip(input.chars()).enumerate() {
			assert_eq!(actual, Some(expected), "char mismatch at index #{i}");
		}
	}

	let input = "Rust really rocks!";
	test_with_str::<32>(input);
	test_with_str::<16>(input);
	test_with_str::<2>(input);
	test_with_str::<1>(input);
	test_with_str::<0>(input);
}

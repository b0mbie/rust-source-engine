use ::std::ffi::CString;

pub fn c_string_from_string(mut value: String) -> CString {
	if let Some(with_nul_len) = value.find('\0').map(add_one) {
		value.truncate(with_nul_len);
		unsafe { CString::from_vec_with_nul_unchecked(value.into_bytes()) }
	} else {
		unsafe { CString::from_vec_unchecked(value.into_bytes()) }
	}
}

pub fn c_string_from_bytes(mut value: Vec<u8>) -> CString {
	if let Some(with_nul_len) = value.iter().position(move |&b| b == b'\0').map(add_one) {
		value.truncate(with_nul_len);
		unsafe { CString::from_vec_with_nul_unchecked(value) }
	} else {
		unsafe { CString::from_vec_unchecked(value) }
	}
}

const fn add_one(value: usize) -> usize {
	value.wrapping_add(1)
}

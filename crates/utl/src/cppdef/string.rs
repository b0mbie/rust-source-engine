use ::core::ffi::c_char;

::rse_cpp::class! {
	#[derive(Debug)]
	pub CUtlString {
		pub string: *mut c_char;
	}
}

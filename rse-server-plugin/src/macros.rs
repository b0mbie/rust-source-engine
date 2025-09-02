/// Generates a [`CStr`](::core::ffi::CStr) that can be used as a description for a server plugin
/// by concatenating the crate's name and version.
/// 
/// For example, given the `plugin` crate of version `0.1.0`, this macro will generate the C string `plugin 0.1.0`.
#[macro_export]
macro_rules! plugin_description {
	() => {
		unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(
			::core::concat!(::core::env!("CARGO_PKG_NAME"), ' ', ::core::env!("CARGO_PKG_VERSION"), "\0").as_bytes()
		) }
	};
}

use ::core::ffi::{
	c_char, c_int,
};

use super::CUtlMap;

::rse_cpp::class! {
	#[derive(Debug)]
	pub CUtlDict<T, I = c_int> {
		pub elements: CUtlMap<*const c_char, T, I>;
	}
}

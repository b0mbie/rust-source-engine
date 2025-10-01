use ::core::ffi::{
	CStr, c_char,
};

use crate::{
	vtable, VtObjectPtr,
};

vtable! {
	GccTypeInfoVt {
		pub fn destructor();
		pub fn destructor_2();
		pub fn __do_upcast() -> bool;
		pub fn __do_catch() -> bool;
		pub fn __do_upcast_2() -> bool;
		pub fn __find_public_src() -> SubKind;
		pub fn __do_dyncast() -> bool;
		pub fn __do_find_public_src() -> SubKind;
	}
}

#[repr(C)]
enum SubKind {
	Unknown = 0,
}

#[repr(C)]
pub struct TypeInfo {
	vtable: *const GccTypeInfoVt,
	name: *const c_char,
}

type This = VtObjectPtr<GccTypeInfoVt>;
unsafe extern "C" fn noop(_: This) {}
unsafe extern "C" fn noop_false(_: This) -> bool {
	false
}
unsafe extern "C" fn noop_unknown(_: This) -> SubKind {
	SubKind::Unknown
}

impl TypeInfo {
	const VTABLE: &GccTypeInfoVt = &GccTypeInfoVt {
		destructor: noop,
		destructor_2: noop,
		__do_upcast: noop_false,
		__do_catch: noop_false,
		__do_upcast_2: noop_false,
		__find_public_src: noop_unknown,
		__do_dyncast: noop_false,
		__do_find_public_src: noop_unknown,
	};

	pub const fn new(name: &'static CStr) -> Self {
		Self {
			vtable: Self::VTABLE,
			name: name.as_ptr(),
		}
	}
}

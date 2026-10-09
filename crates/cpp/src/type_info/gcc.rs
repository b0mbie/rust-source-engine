use ::core::ffi::{
	CStr, c_char,
};

use crate::{
	class, virtual_fn,
};

class! {
	pub GccTypeInfo {
		pub virtual fn destructor();
		pub virtual fn destructor_2();
		pub virtual fn __do_upcast() -> bool;
		pub virtual fn __do_catch() -> bool;
		pub virtual fn __do_upcast_2() -> bool;
		pub virtual fn __find_public_src() -> SubKind;
		pub virtual fn __do_dyncast() -> bool;
		pub virtual fn __do_find_public_src() -> SubKind;
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

type This = crate::This<GccTypeInfo>;

virtual_fn! {
	fn noop(_: This) {}
}
virtual_fn! {
	fn noop_false(_: This) -> bool { false }
}
virtual_fn! {
	fn noop_unknown(_: This) -> SubKind { SubKind::Unknown }
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

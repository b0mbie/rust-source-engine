use ::core::ffi::{
	CStr, c_char, c_float, c_int, c_void,
};
use ::rse_cpp::{
	wide_c_str::{
		WCStr, wchar_t,
	},
	VTablePtr, vtable_methods, new_vtable_self,
	This, this_to_self,
};

use crate::cppdef::{
	IGameEventVisitor2, IGameEventVisitor2Vt,
};

pub trait VisitEvent<'e> {
	// TODO: What is "local" supposed to mean, exactly?
	fn visit_local(&mut self, name: &CStr, value: *const ()) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_c_str(&mut self, name: &CStr, value: &'e CStr) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_float(&mut self, name: &CStr, value: c_float) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_int(&mut self, name: &CStr, value: c_int) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_u64(&mut self, name: &CStr, value: u64) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_wcstr(&mut self, name: &CStr, value: &'e WCStr) -> bool {
		let _ = name;
		let _ = value;
		true
	}
	fn visit_bool(&mut self, name: &CStr, value: bool) -> bool {
		let _ = name;
		let _ = value;
		true
	}
}

#[repr(C)]
pub struct VisitorObject<T> {
	raw: IGameEventVisitor2,
	inner: T,
}

impl<'a, T: VisitEvent<'a>> VisitorObject<T> {
	pub const fn new(inner: T) -> Self {
		Self {
			raw: IGameEventVisitor2 {
				vtable: VTablePtr::from_ref(Self::VTABLE),
			},
			inner,
		}
	}

	pub const fn as_raw(&self) -> &IGameEventVisitor2 {
		&self.raw
	}

	pub const unsafe fn as_mut_raw(&mut self) -> &mut IGameEventVisitor2 {
		&mut self.raw
	}

	const VTABLE: &'static IGameEventVisitor2Vt = &new_vtable_self!(IGameEventVisitor2Vt {
		visit_local,
		visit_string,
		visit_float,
		visit_int,
		visit_uint64,
		visit_w_string,
		visit_bool,
	});

	vtable_methods! {
		this: This<IGameEventVisitor2>;
		fn visit_local(name: *const c_char, value: *const c_void) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_local(name, value as _)
		}
		fn visit_string(name: *const c_char, value: *const c_char) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			let value = unsafe { CStr::from_ptr(value) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_c_str(name, value)
		}
		fn visit_float(name: *const c_char, value: c_float) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_float(name, value)
		}
		fn visit_int(name: *const c_char, value: c_int) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_int(name, value)
		}
		fn visit_uint64(name: *const c_char, value: u64) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_u64(name, value)
		}
		fn visit_w_string(name: *const c_char, value: *const wchar_t) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			let value = unsafe { WCStr::from_ptr(value) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_wcstr(name, value)
		}
		fn visit_bool(name: *const c_char, value: bool) -> bool {
			let name = unsafe { CStr::from_ptr(name) };
			unsafe { this_to_self!(mut this) }
				.inner.visit_bool(name, value)
		}
	}
}

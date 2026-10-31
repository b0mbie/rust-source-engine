use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	ptr::null,
};
use ::rse_cpp::{
	wide_c_str::WCStr,
	transparent_wrapper, virtual_call,
};

use crate::{
	cppdef::IGameEvent,
	KeyValues,
};

use super::{
	VisitEvent, VisitorObject,
};

transparent_wrapper! {
	pub struct GameEvent for IGameEvent as "IGameEvent";
}

impl GameEvent {
	pub fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_inner() => get_name())) }
	}

	pub fn is_reliable(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_reliable()) }
	}
	pub fn is_local(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_local()) }
	}

	pub fn is_key_empty(&self, key: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_empty(key.as_ptr())) }
	}

	pub fn get_bool_or(&self, key: &CStr, default: bool) -> bool {
		unsafe { virtual_call!(self.as_inner() => get_bool(key.as_ptr(), default)) }
	}
	pub fn get_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_int(key.as_ptr(), default)) }
	}
	pub fn get_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_inner() => get_float(key.as_ptr(), default)) }
	}
	pub fn get_c_str_or<'a>(&'a self, key: &CStr, default: &'a CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_string(key.as_ptr(), default.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	pub fn get_c_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_string(key.as_ptr(), null())) };
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}
	pub fn get_u64_or(&self, key: &CStr, default: u64) -> u64 {
		unsafe { virtual_call!(self.as_inner() => get_uint64(key.as_ptr(), default)) }
	}
	pub fn get_ptr<T>(&self, key: &CStr) -> *const T {
		(unsafe { virtual_call!(self.as_inner() => get_ptr(key.as_ptr())) }) as _
	}

	pub fn get_wcstr_or<'a>(&'a self, key: &CStr, default: &'a WCStr) -> &'a WCStr {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_w_string(key.as_ptr(), default.as_ptr())) };
		unsafe { WCStr::from_ptr(ptr) }
	}
	pub fn get_wcstr<'a>(&'a self, key: &CStr) -> Option<&'a WCStr> {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_w_string(key.as_ptr(), null())) };
		if !ptr.is_null() {
			unsafe { Some(WCStr::from_ptr(ptr)) }
		} else {
			None
		}
	}

	pub fn set_bool(&mut self, key: &CStr, value: bool) {
		unsafe { virtual_call!(self.as_mut_inner() => set_bool(key.as_ptr(), value)) }
	}
	pub fn set_int(&mut self, key: &CStr, value: c_int) {
		unsafe { virtual_call!(self.as_mut_inner() => set_int(key.as_ptr(), value)) }
	}
	pub fn set_float(&mut self, key: &CStr, value: c_float) {
		unsafe { virtual_call!(self.as_mut_inner() => set_float(key.as_ptr(), value)) }
	}
	pub fn set_c_str(&mut self, key: &CStr, value: &CStr) {
		unsafe { virtual_call!(self.as_mut_inner() => set_string(key.as_ptr(), value.as_ptr())) }
	}
	pub fn set_u64(&mut self, key: &CStr, value: u64) {
		unsafe { virtual_call!(self.as_mut_inner() => set_uint64(key.as_ptr(), value)) }
	}
	pub fn set_ptr<T>(&mut self, key: &CStr, value: *const T) {
		unsafe { virtual_call!(self.as_mut_inner() => set_ptr(key.as_ptr(), value as _)) }
	}
	pub fn set_wcstr(&mut self, key: &CStr, value: &WCStr) {
		unsafe { virtual_call!(self.as_mut_inner() => set_w_string(key.as_ptr(), value.as_ptr())) }
	}

	// TODO: Nullable?
	pub fn get_data_keys(&mut self) -> Option<&mut KeyValues> {
		unsafe {
			let raw = virtual_call!(self.as_mut_inner() => get_data_keys()).as_mut()?;
			Some(KeyValues::from_mut(raw))
		}
	}

	pub fn visit<'a, F: FnMut(&CStr, EventValue<'a>) -> bool>(&'a self, f: F) -> bool {
		use ::core::marker::PhantomData;
		struct FnVisitor<'a, F>(F, PhantomData<&'a ()>);
		impl<'a, F: FnMut(&CStr, EventValue<'a>) -> bool> VisitEvent<'a> for FnVisitor<'a, F> {
			fn visit_local(&mut self, name: &CStr, value: *const ()) -> bool {
				self.0(name, EventValue::Ptr(value))
			}
			fn visit_c_str(&mut self, name: &CStr, value: &'a CStr) -> bool {
				self.0(name, EventValue::CStr(value))
			}
			fn visit_float(&mut self, name: &CStr, value: c_float) -> bool {
				self.0(name, EventValue::Float(value))
			}
			fn visit_int(&mut self, name: &CStr, value: c_int) -> bool {
				self.0(name, EventValue::Int(value))
			}
			fn visit_u64(&mut self, name: &CStr, value: u64) -> bool {
				self.0(name, EventValue::U64(value))
			}
			fn visit_wcstr(&mut self, name: &CStr, value: &'a WCStr) -> bool {
				self.0(name, EventValue::WCStr(value))
			}
			fn visit_bool(&mut self, name: &CStr, value: bool) -> bool {
				self.0(name, EventValue::Bool(value))
			}
		}
		self.visit_with(FnVisitor(f, PhantomData))
	}

	pub fn visit_with<'a, V: VisitEvent<'a>>(&'a self, mut visitor: V) -> bool {
		struct DynVisitor<'a, 'e>(&'a mut dyn VisitEvent<'e>);
		macro_rules! delegates {
			{
				$(
					fn $name:ident(&mut self $(, $param:ident: $Param:ty)* $(,)?);
				)*
			} => {
				$(
					fn $name(&mut self $(, $param: $Param)*) -> bool {
						self.0.$name($($param,)*)
					}
				)*
			};
		}
		impl<'e> VisitEvent<'e> for DynVisitor<'_, 'e> {
			delegates! {
				fn visit_local(&mut self, name: &CStr, value: *const ());
				fn visit_c_str(&mut self, name: &CStr, value: &'e CStr);
				fn visit_float(&mut self, name: &CStr, value: c_float);
				fn visit_int(&mut self, name: &CStr, value: c_int);
				fn visit_u64(&mut self, name: &CStr, value: u64);
				fn visit_wcstr(&mut self, name: &CStr, value: &'e WCStr);
				fn visit_bool(&mut self, name: &CStr, value: bool);
			}
		}
		self.visit_generic(DynVisitor(&mut visitor))
	}

	pub fn visit_generic<'a, V: VisitEvent<'a>>(&'a self, visitor: V) -> bool {
		let mut object = VisitorObject::new(visitor);
		unsafe { virtual_call!(self.as_inner() => for_event_data(object.as_mut_raw())) }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum EventValue<'a> {
	Ptr(*const ()),
	CStr(&'a CStr),
	Float(c_float),
	Int(c_int),
	U64(u64),
	WCStr(&'a WCStr),
	Bool(bool),
}

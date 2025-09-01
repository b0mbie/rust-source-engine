use ::core::{
	ffi::CStr,
	marker::PhantomData,
	ptr::null_mut,
};
use ::rse_cpp::{
	AsObject, virtual_call, owned_vt_object_wrapper,
};
use ::rse_game::cppdef::entities::ServerClass as CServerClass;

use crate::{
	cppdef::{
		ServerGameDllVt, INTERFACEVERSION_SERVERGAMEDLL,
		TickInterval,
	},
	InterfaceOfFactory, GameServerFactory,
};

pub trait ServerGameDllImpl: AsObject<ServerGameDllVt> {
	fn tick_interval(&self) -> TickInterval {
		unsafe { virtual_call!(self.as_object() => get_tick_interval()) } 
	}
	fn server_classes(&self) -> ServerClasses<'_> {
		unsafe { ServerClasses {
			head: Some(ServerClass::from_ptr(virtual_call!(self.as_object() => get_all_server_classes()))),
		} }
	}
	fn server_classes_mut(&mut self) -> ServerClassesMut<'_> {
		unsafe { ServerClassesMut {
			head: virtual_call!(self.as_object() => get_all_server_classes()),
			_life: PhantomData,
		} }
	}
}
impl<T: AsObject<ServerGameDllVt>> ServerGameDllImpl for T {}

owned_vt_object_wrapper! {
	pub struct ServerGameDll for ServerGameDllVt;
}
unsafe impl ::rse_interface::Interface for ServerGameDll {
	const IDENTIFIER: &CStr = INTERFACEVERSION_SERVERGAMEDLL;
}
impl InterfaceOfFactory for ServerGameDll {
	type Factory = GameServerFactory;
}

/// # Layout
/// This type has the exact same layout as a C++ [`ServerClass`](CServerClass).
#[repr(transparent)]
pub struct ServerClass(CServerClass);

impl ServerClass {
	/// # Safety
	/// `ptr` must point to a valid, mutable [`ServerClass`](CServerClass).
	pub const unsafe fn from_ptr_mut<'a>(ptr: *mut CServerClass) -> &'a mut Self {
		unsafe { &mut *(ptr as *mut Self) }
	}

	/// # Safety
	/// `ptr` must point to a valid, immutable [`ServerClass`](CServerClass).
	pub const unsafe fn from_ptr<'a>(ptr: *const CServerClass) -> &'a Self {
		unsafe { &*(ptr as *const Self) }
	}

	pub const fn as_ptr_mut(&mut self) -> *mut CServerClass {
		self as *mut Self as *mut CServerClass
	}

	pub const fn as_ptr(&self) -> *const CServerClass {
		self as *const Self as *const CServerClass
	}

	pub const fn network_name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.network_name) }
	}

	pub fn next(&self) -> Option<&Self> {
		let next = self.0.next;
		if !next.is_null() {
			unsafe { Some(Self::from_ptr(next)) }
		} else {
			None
		}
	}

	pub fn next_mut(&mut self) -> Option<&mut Self> {
		let next = self.0.next;
		if !next.is_null() {
			unsafe { Some(Self::from_ptr_mut(next)) }
		} else {
			None
		}
	}
}

#[repr(transparent)]
pub struct ServerClasses<'a> {
	head: Option<&'a ServerClass>,
}
impl<'a> Iterator for ServerClasses<'a> {
	type Item = &'a ServerClass;
	fn next(&mut self) -> Option<Self::Item> {
		let element = self.head?;
		self.head = element.next();
		Some(element)
	}
}

#[repr(transparent)]
pub struct ServerClassesMut<'a> {
	head: *mut CServerClass,
	_life: PhantomData<&'a mut ServerClass>,
}
impl<'a> Iterator for ServerClassesMut<'a> {
	type Item = &'a mut ServerClass;
	fn next(&mut self) -> Option<Self::Item> {
		let head = self.head;
		if !head.is_null() {
			unsafe {
				let class = ServerClass::from_ptr_mut(head);
				self.head = class.next_mut().map(move |c| c.as_ptr_mut()).unwrap_or(null_mut());
				Some(class)
			}
		} else {
			None
		}
	}
}

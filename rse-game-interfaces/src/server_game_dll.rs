use ::core::{
	ffi::{
		CStr, c_int,
	},
	marker::PhantomData,
	ptr::null_mut,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};
use ::rse_cpp::{
	AsObject, virtual_call, owned_vt_object_wrapper,
	transparent_wrapper,
};
use ::rse_game::cppdef::{
	datatable::{
		SendProp as CSendProp,
		SendTable as CSendTable,
	},
	entities::ServerClass as CServerClass,
};

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

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout as a C++ [`SendProp`](CSendProp).
	pub struct SendProp for CSendProp as "SendProp";
}

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout as a C++ [`SendTable`](CSendTable).
	pub struct SendTable for CSendTable as "SendTable";
}

impl SendTable {
	pub const fn props(&self) -> &[SendProp] {
		unsafe { from_raw_parts(self.0.props as *const SendProp, self.0.n_props as usize) }
	}

	pub const fn props_mut(&mut self) -> &mut [SendProp] {
		unsafe { from_raw_parts_mut(self.0.props as *mut SendProp, self.0.n_props as usize) }
	}
}

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout as a C++ [`ServerClass`](CServerClass).
	pub struct ServerClass for CServerClass as "ServerClass";
}

impl ServerClass {
	pub const fn network_name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.network_name) }
	}

	pub const fn class_id(&self) -> c_int {
		self.0.class_id
	}

	pub const fn table(&self) -> &SendTable {
		unsafe { SendTable::from_ptr(self.0.table) }
	}

	pub const fn table_mut(&mut self) -> &mut SendTable {
		unsafe { SendTable::from_ptr_mut(self.0.table) }
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

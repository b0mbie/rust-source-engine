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
use ::rse_cpp::transparent_wrapper;

use crate::cppdef::{
	datatable::{
		SendProp as CSendProp,
		SendTable as CSendTable,
	},
	entities::ServerClass as CServerClass,
};

pub use crate::cppdef::datatable::{
	SendPropType, ValueLimit,
};

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout as a C++ [`SendProp`](CSendProp).
	pub struct SendProp for CSendProp as "SendProp";
}

impl SendProp {
	pub const fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.var_name) }
	}

	pub const fn prop_type(&self) -> SendPropType {
		self.0.prop_type
	}

	pub const fn offset(&self) -> isize {
		self.0.offset as _
	}

	pub const fn table(&self) -> Option<&SendTable> {
		let ptr = self.0.data_table;
		if !ptr.is_null() {
			unsafe { Some(SendTable::from_ptr(ptr)) }
		} else {
			None
		}
	}

	pub const fn table_mut(&mut self) -> Option<&mut SendTable> {
		let ptr = self.0.data_table;
		if !ptr.is_null() {
			unsafe { Some(SendTable::from_ptr_mut(ptr)) }
		} else {
			None
		}
	}

	/// # Safety
	/// The [`SendProp`] must be a [`SendPropType::DataTable`].
	pub const unsafe fn table_unchecked(&self) -> &SendTable {
		unsafe { SendTable::from_ptr(self.0.data_table) }
	}

	/// # Safety
	/// The [`SendProp`] must be a [`SendPropType::DataTable`].
	pub const unsafe fn table_unchecked_mut(&mut self) -> &mut SendTable {
		unsafe { SendTable::from_ptr_mut(self.0.data_table) }
	}

	pub const fn low_value(&self) -> ValueLimit {
		self.0.low_value
	}

	pub const fn high_value(&self) -> ValueLimit {
		self.0.high_value
	}

	pub const fn n_bits(&self) -> usize {
		self.0.bits as _
	}
}

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout as a C++ [`SendTable`](CSendTable).
	pub struct SendTable for CSendTable as "SendTable";
}

impl SendTable {
	pub const fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.net_table_name) }
	}

	pub const fn n_props(&self) -> usize {
		self.0.n_props as usize
	}

	pub const fn props(&self) -> &[SendProp] {
		unsafe { from_raw_parts(self.0.props as *const SendProp, self.n_props()) }
	}

	pub const fn props_mut(&mut self) -> &mut [SendProp] {
		unsafe { from_raw_parts_mut(self.0.props as *mut SendProp, self.n_props()) }
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

impl<'a> ServerClasses<'a> {
	pub const fn new(head: &'a ServerClass) -> Self {
		Self {
			head: Some(head),
		}
	}
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

impl<'a> ServerClassesMut<'a> {
	pub const fn new(head: &'a mut ServerClass) -> Self {
		Self {
			head: head.as_ptr_mut(),
			_life: PhantomData,
		}
	}
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

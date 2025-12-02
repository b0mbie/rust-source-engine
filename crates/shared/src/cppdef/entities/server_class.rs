use ::core::ffi::{
	c_char, c_int,
};

use crate::cppdef::datatable::SendTable;

#[derive(Debug)]
#[repr(C)]
pub struct ServerClass {
	pub network_name: *const c_char,
	pub table: *mut SendTable,
	pub next: *mut Self,
	pub class_id: c_int,
	pub instance_baseline_index: c_int,
}

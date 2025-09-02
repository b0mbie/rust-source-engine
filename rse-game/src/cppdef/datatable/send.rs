use ::core::ffi::{
	c_char, c_float, c_int, c_void,
};
use ::rse_cpp::vtable;

use super::{
	Variant, SendPropType, SendPropFlags,
};

// TODO: `RecvProp`.
pub type RecvProp = c_void;

// TODO: `SendTablePrecalc`.
pub type SendTablePrecalc = c_void;

// TODO: `CSendProxyRecipients`.
pub type SendProxyRecipients = c_void;

pub type ArrayLengthSendProxyFn = unsafe extern "C" fn(the_struct: *const c_void, object_id: c_int) -> c_int;
pub type SendVarProxyFn = unsafe extern "C" fn(
	prop: *const SendProp,
	base: *const c_void, data: *const c_void,
	out: *mut Variant,
	element: c_int, object_id: c_int,
);
pub type SendTableProxyFn = unsafe extern "C" fn(
	prop: *const SendProp,
	base: *const c_void, data: *const c_void,
	recipients: *mut SendProxyRecipients,
	object_id: c_int,
) -> *mut c_void;

#[repr(C)]
pub struct SendTable {
	pub props: *mut SendProp,
	pub n_props: c_int,
	pub net_table_name: *const c_char,
	pub precalc: *mut SendTablePrecalc,
}

vtable! {
	pub SendTableVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
	}
}

#[repr(C)]
pub struct SendProp {
	pub vtable: *mut SendTableVt,
	pub matching_recv_prop: *mut RecvProp,
	pub prop_type: SendPropType,
	pub bits: c_int,
	pub low_value: ValueLimit,
	pub high_value: ValueLimit,
	pub array_prop: *mut SendProp,
	pub array_length_proxy: Option<ArrayLengthSendProxyFn>,
	pub n_elements: c_int,
	pub element_stride: c_int,
	pub exclude_dt_name: *const c_char,
	pub parrent_array_prop_name: *const c_char,
	pub var_name: *const c_char,
	pub high_low_mul: c_float,
	pub flags: SendPropFlags,
	pub proxy_fn: Option<SendVarProxyFn>,
	pub dt_proxy_fn: Option<SendTableProxyFn>,
	pub data_table: *mut SendTable,
	pub offset: c_int,
	pub extra_data: *const c_void,
}

pub type ValueLimit = c_float;

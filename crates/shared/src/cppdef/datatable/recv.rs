use ::core::ffi::{
	c_char, c_int, c_void,
};
use ::rse_cpp::class;

use super::{
	SendPropType,
	ArrayLengthSendProxyFn,
	DVariant,
};

class! {
	pub RecvProp {
		pub var_name: *const c_char;
		pub recv_type: SendPropType;
		pub flags: c_int;
		pub string_buffer_size: c_int;
		pub inside_array: bool;
		pub extra_data: *const c_void;
		pub array_prop: *mut RecvProp;
		pub array_length_proxy: ArrayLengthSendProxyFn;
		pub proxy_fn: RecvVarProxyFn;
		pub data_table_proxy_fn: DataTableRecvVarProxyFn;
		pub data_table: *mut RecvTable;
		pub offset: c_int;
		pub element_stride: c_int;
		pub n_elements: c_int;
		pub parent_array_prop_name: *const c_char;
	}
}

class! {
	pub RecvTable {
		pub props: *mut RecvProp;
		pub n_props: c_int;
		pub decoder: *mut CRecvDecoder;
		pub net_table_name: *const c_char;
		pub initialized: bool;
		pub in_main_list: bool;
	}
}

class! {
	// TODO: `CRecvDecoder`.
	pub CRecvDecoder {

	}
}

pub type RecvVarProxyFn = unsafe extern "C" fn(
	data: *const CRecvProxyData,
	the_struct: *mut c_void, out: *mut c_void,
);
pub type ArrayLengthRecvProxyFn	= unsafe extern "C" fn(
	the_struct: *mut c_void, object_id: c_int, current_array_length: c_int,
);
pub type DataTableRecvVarProxyFn = unsafe extern "C" fn(
	prop: *const RecvProp, out: *mut *mut c_void, data: *mut c_void,
	object_id: c_int,
);

class! {
	pub CRecvProxyData {
		pub recv_prop: *const RecvProp;
		pub value: DVariant;
		pub element: c_int;
		pub object_id: c_int;
	}
}

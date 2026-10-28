use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::class;
use ::rse_shared::cppdef::datatable::RecvTable;

class! {
	pub ClientClass {
		pub create_fn: CreateClientClassFn;
		pub create_event_fn: CreateEventFn;
		pub network_name: *const c_char;
		pub recv_table: *mut RecvTable;
		pub next: *mut ClientClass;
		pub class_id: c_int;
	}
}

class! {
	// TODO: `IClientNetworkable`?
	pub IClientNetworkable {}
}

class! {
	// TODO: `IClientUnknown`?
	pub IClientUnknown {}
}

pub type CreateClientClassFn = unsafe extern "C" fn(ent_num: c_int, serial_num: c_int) -> *mut IClientNetworkable;
pub type CreateEventFn = unsafe extern "C" fn() -> *mut IClientNetworkable;

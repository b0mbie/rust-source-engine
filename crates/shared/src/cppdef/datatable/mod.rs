use ::core::ffi::{
	CStr, c_float, c_int,
};

mod recv;
pub use recv::*;
mod send;
pub use send::*;
mod sprop_flags;
pub use sprop_flags::*;
mod variant;
pub use variant::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RsVariant<'a> {
	Int(c_int),
	Float(c_float),
	Vector([c_float; 2]),
	VectorXy([c_float; 3]),
	String(&'a CStr),
	Array,
	DataTable,
	#[cfg(feature = "datatable-supports-int64")]
	Int64(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SendPropType {
	Int,
	Float,
	Vector,
	VectorXy,
	String,
	Array,
	DataTable,
	#[cfg(feature = "datatable-supports-int64")]
	Int64,
}

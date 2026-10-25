use ::core::ffi::{
	c_char, c_float, c_int, c_short, c_ushort,
};
use ::rse_cpp::RefMut;

#[repr(C)]
pub struct datamap_t {
	pub data_desc: *mut typedescription_t,
	pub data_num_fields: c_int,
	pub data_class_name: *const c_char,
	pub base_map: *mut Self,
	pub chains_validated: bool,
	pub packed_offsets_computed: bool,
	pub packed_size: c_int,
}

#[repr(C)]
pub struct typedescription_t {
	pub field_type: fieldtype_t,
	pub field_name: *const c_char,
	pub field_offset: [c_int; TdOffset::Count as _],
	pub field_size: c_ushort,
	pub flags: c_short,
	pub external_name: *const c_char,
	pub save_restore_ops: *mut ISaveRestoreOps,
	pub input_func: inputfunc_t,
	pub td: *mut Self,
	pub field_size_in_bytes: c_int,
	pub override_field: *mut Self,
	pub override_count: c_int,
	pub field_tolerance: c_float,
}

#[allow(non_camel_case_types)]
pub type inputfunc_t = unsafe extern "C" fn(data: RefMut<inputdata_t>);

#[repr(C)]
pub struct inputdata_t {
	pub field_type: fieldtype_t,
	pub field_name: *const c_char,
	pub field_offset: [c_int; TdOffset::Count as _],
	pub field_size: c_ushort,
	pub flags: c_short,
	pub external_name: *const c_char,
	pub save_restore_ops: *mut ISaveRestoreOps,
}

::rse_cpp::class! {
	// TODO: `ISaveRestoreOps`.
	pub ISaveRestoreOps {}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum TdOffset {
	Normal = 0,
	Packed = 1,
	// Must be last.
	Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum fieldtype_t {
	Void = 0,
	Float,
	String,
	Vector,
	Quaternion,
	Integer,
	Boolean,
	Short,
	Character,
	Color32,
	Embedded,
	Custom,

	ClassPtr,
	EHandle,
	Edict,

	PositionVector,
	Time,
	Tick,
	ModelName,
	SoundName,

	Input,
	Function,

	VMatrix,

	VMatrixWorldSpace,
	Matrix3x4WorldSpace,

	Interval,
	ModelIndex,
	MaterialIndex,

	Vector2d,

	TypeCount,
}

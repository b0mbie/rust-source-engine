use ::core::{
	ffi::{
		c_char, c_float, c_int, c_short, c_uint, c_ulong, c_ushort, c_void,
	},
	ptr::null_mut,
};
use ::rse_cpp::{
	RefConst, RefMut,
	class,
};
use ::rse_math::{
	Vector, QAngle, matrix3x4_t,
};

use super::{
	model_t, SolidType,
	Ray_t, trace_t,
	string_t,
};

mod server_class;
pub use server_class::*;
mod state_flags;
pub use state_flags::*;

pub type EdictIndex = c_short;

class! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub CBaseEdict {
		pub state_flags: StateFlags;
		pub network_serial_number: c_short;
		pub edict_index: EdictIndex;
		pub networkable: *mut IServerNetworkable;
		pub unknown: *mut IServerUnknown;
	}
}

impl CBaseEdict {
	pub const fn server_entity(self) -> *mut IServerEntity {
		if self.state_flags.is_full_entity() {
			self.unknown.cast()
		} else {
			null_mut()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct edict_t {
	pub base_edict: CBaseEdict,
	pub free_time: c_float,
}

// TODO: `CBaseNetworkable`.
pub type CBaseNetworkable = c_void;

/// Number of clusters an entity can span
/// before a slower area checking algorithm is used.
pub const MAX_FAST_ENT_CLUSTERS: usize = 4;

/// Information necessary to perform PVS testing.
#[derive(Debug)]
#[repr(C)]
pub struct PVSInfo_t {
	pub head_node: c_short,
	pub cluster_count: c_short,
	pub clusters: *mut c_ushort,
	pub area_num: c_short,
	pub area_num_2: c_short,
	pub center: [c_float; 3],
	pub clusters_inline: [c_ushort; MAX_FAST_ENT_CLUSTERS],
}

class! {
	pub IServerNetworkable {
		pub virtual fn get_entity_handle() -> *mut IHandleEntity;
		pub virtual fn get_server_class() -> *mut ServerClass;
		pub virtual fn get_edict() -> *mut edict_t;
		pub virtual fn get_class_name() -> *const c_char;
		pub virtual fn release();
		pub virtual fn area_num() -> c_int;
		pub virtual fn get_base_networkable() -> *mut CBaseNetworkable;
		pub virtual fn get_base_entity() -> *mut CBaseEntity;
		pub virtual fn get_pvs_info() -> *mut PVSInfo_t;
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
	}
}

class! {
	pub IServerEntity: pub virtual IServerUnknown {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
		pub virtual fn get_model_index() -> c_int;
		pub virtual fn get_model_name() -> string_t;
		pub virtual fn set_model_index(index: c_int);
	}
}

class! {
	pub IServerUnknown: pub virtual IHandleEntity {
		pub virtual fn get_collideable() -> *mut ICollideable;
		pub virtual fn get_networkable() -> *mut IServerNetworkable;
		pub virtual fn get_base_entity() -> *mut CBaseEntity;
	}
}

// TODO: `IClientUnknown`?
class! {
	pub IClientUnknown {}
}

class! {
	pub ICollideable {
		pub virtual fn get_entity_handle() -> *mut IHandleEntity;
		pub virtual fn obb_mins_pre_scaled() -> RefConst<Vector>;
		pub virtual fn obb_maxs_pre_scaled() -> RefConst<Vector>;
		pub virtual fn obb_mins() -> RefConst<Vector>;
		pub virtual fn obb_maxs() -> RefConst<Vector>;
		pub virtual fn world_space_trigger_bounds(out_world_mins: *mut Vector, out_world_maxs: *mut Vector);
		pub virtual fn test_collision(ray: RefConst<Ray_t>, contents_mask: c_uint, out_trace: RefMut<trace_t>) -> bool;
		pub virtual fn test_hitboxes(ray: RefConst<Ray_t>, contents_mask: c_uint, out_trace: RefMut<trace_t>) -> bool;
		pub virtual fn get_collision_model_index() -> c_int;
		pub virtual fn get_collision_model() -> *const model_t;
		pub virtual fn get_collision_origin() -> RefConst<Vector>;
		pub virtual fn get_collision_angles() -> RefConst<QAngle>;
		pub virtual fn collision_to_world_transform() -> RefConst<matrix3x4_t>;
		pub virtual fn get_solid() -> SolidType;
		pub virtual fn get_solid_flags() -> c_int;
		pub virtual fn get_client_unknown() -> *mut IClientUnknown;
		pub virtual fn get_collision_group() -> c_int;
		pub virtual fn world_space_surrounding_bounds(out_mins: *mut Vector, out_maxs: *mut Vector);
		pub virtual fn should_touch_trigger(trigger_solid_flags: c_int) -> bool;
		pub virtual fn get_root_parent_to_world_transform() -> *const matrix3x4_t;
	}
}

class! {
	// TODO: `CBaseEntity`.
	pub CBaseEntity {}
}

class! {
	pub CHandle<T>: pub CBaseHandle {
		pub use T as _t;
	}
}

class! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub CBaseHandle {
		pub index: c_ulong;
	}
}

impl Default for CBaseHandle {
	fn default() -> Self {
		Self::INVALID
	}
}

/// Number of bits needed to represent max edicts.
pub const MAX_EDICT_BITS: u8 = 11;

/// Maximum number of edicts on a server.
pub const MAX_EDICTS: usize = 1 << MAX_EDICT_BITS;

impl CBaseHandle {
	const NUM_ENT_ENTRY_BITS: u8 = MAX_EDICT_BITS + 1;
	const NUM_ENT_ENTRIES: usize = 1 << Self::NUM_ENT_ENTRY_BITS;
	const ENT_ENTRY_MASK: c_ulong = (Self::NUM_ENT_ENTRIES as c_ulong) - 1;

	pub const INVALID: Self = Self {
		index: 0xFFFFFFFF,
	};

	pub const fn new(entry: c_ulong, serial_number: c_ulong) -> Self {
		Self {
			index: entry | (serial_number << Self::NUM_ENT_ENTRY_BITS),
		}
	}

	pub const fn entry_index(&self) -> c_ulong {
		self.index & Self::ENT_ENTRY_MASK
	}

	pub const fn serial_number(&self) -> c_ulong {
		self.index >> Self::NUM_ENT_ENTRY_BITS
	}
}

class! {
	pub IHandleEntity {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
		pub virtual fn set_ref_ehandle(handle: RefConst<CBaseHandle>);
		pub virtual fn get_ref_ehandle() -> RefConst<CBaseHandle>;
	}
}

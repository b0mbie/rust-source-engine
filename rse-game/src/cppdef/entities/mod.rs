use ::core::ffi::{
	c_char, c_float, c_int, c_short, c_uint, c_ulong, c_ushort, c_void,
};
use ::rse_cpp::{
	vtable, VtObjectMut, RefConst, RefMut,
};
use ::rse_math::{
	Vector, QAngle, Matrix3x4,
};

use super::{
	Model, SolidType,
	Ray, Trace,
	string_t,
};

mod server_class;
pub use server_class::*;
mod state_flags;
pub use state_flags::*;

pub type EdictIndex = c_short;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BaseEdict {
	pub state_flags: StateFlags,
	pub network_serial_number: c_short,
	pub edict_index: EdictIndex,
	pub networkable: Option<VtObjectMut<ServerNetworkableVt>>,
	pub unknown: Option<VtObjectMut<ServerUnknownVt>>,
}

impl BaseEdict {
	pub const fn server_entity(self) -> Option<VtObjectMut<ServerEntityVt>> {
		if let Some(unknown) = self.unknown {
			if self.state_flags.is_full_entity() {
				Some(unknown.cast())
			} else {
				None
			}
		} else {
			None
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Edict {
	pub base_edict: BaseEdict,
	pub free_time: c_float,
}

// TODO: `CBaseNetworkable`.
pub type BaseNetworkable = c_void;

/// Number of clusters an entity can span
/// before a slower area checking algorithm is used.
pub const MAX_FAST_ENT_CLUSTERS: usize = 4;

/// Information necessary to perform PVS testing.
#[derive(Debug)]
#[repr(C)]
pub struct PvsInfo {
	pub head_node: c_short,
	pub cluster_count: c_short,
	pub clusters: *mut c_ushort,
	pub area_num: c_short,
	pub area_num_2: c_short,
	pub center: [c_float; 3],
	pub clusters_inline: [c_ushort; MAX_FAST_ENT_CLUSTERS],
}

vtable! {
	pub ServerNetworkableVt {
		pub fn get_entity_handle() -> VtObjectMut<HandleEntityVt>;
		pub fn get_server_class() -> *mut ServerClass;
		pub fn get_edict() -> *mut Edict;
		pub fn get_class_name() -> *const c_char;
		pub fn release();
		pub fn area_num() -> c_int;
		pub fn get_base_networkable() -> *mut BaseNetworkable;
		pub fn get_base_entity() -> *mut BaseEntity;
		pub fn get_pvs_info() -> *mut PvsInfo;
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
	}
}

#[repr(C)]
pub struct ServerEntityVt {
	pub base: ServerUnknownVt,
	pub server_entity: ServerEntityVtBase,
}

vtable! {
	pub ServerEntityVtBase for VtObjectMut<ServerEntityVt> {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn get_model_index() -> c_int;
		pub fn get_model_name() -> string_t;
		pub fn set_model_index(index: c_int);
	}
}

#[repr(C)]
pub struct ServerUnknownVt {
	pub base: HandleEntityVt,
	pub unknown: ServerUnknownVtBase,
}

vtable! {
	pub ServerUnknownVtBase for VtObjectMut<ServerUnknownVt> {
		pub fn get_collideable() -> VtObjectMut<CollideableVt>;
		pub fn get_networkable() -> VtObjectMut<ServerNetworkableVt>;
		pub fn get_base_entity() -> *mut BaseEntity;
	}
}

// TODO: `IClientUnknown`?
vtable! {
	pub ClientUnknownVt {}
}

vtable! {
	pub CollideableVt {
		pub fn get_entity_handle() -> VtObjectMut<HandleEntityVt>;
		pub fn obb_mins_pre_scaled() -> RefConst<Vector>;
		pub fn obb_maxs_pre_scaled() -> RefConst<Vector>;
		pub fn obb_mins() -> RefConst<Vector>;
		pub fn obb_maxs() -> RefConst<Vector>;
		pub fn world_space_trigger_bounds(out_world_mins: *mut Vector, out_world_maxs: *mut Vector);
		pub fn test_collision(ray: RefConst<Ray>, contents_mask: c_uint, out_trace: RefMut<Trace>) -> bool;
		pub fn test_hitboxes(ray: RefConst<Ray>, contents_mask: c_uint, out_trace: RefMut<Trace>) -> bool;
		pub fn get_collision_model_index() -> c_int;
		pub fn get_collision_model() -> *const Model;
		pub fn get_collision_origin() -> RefConst<Vector>;
		pub fn get_collision_angles() -> RefConst<QAngle>;
		pub fn collision_to_world_transform() -> RefConst<Matrix3x4>;
		pub fn get_solid() -> SolidType;
		pub fn get_solid_flags() -> c_int;
		pub fn get_client_unknown() -> Option<VtObjectMut<ClientUnknownVt>>;
		pub fn get_collision_group() -> c_int;
		pub fn world_space_surrounding_bounds(out_mins: *mut Vector, out_maxs: *mut Vector);
		pub fn should_touch_trigger(trigger_solid_flags: c_int) -> bool;
		pub fn get_root_parent_to_world_transform() -> *const Matrix3x4;
	}
}

// TODO: `CBaseEntity`.
pub type BaseEntity = c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BaseHandle {
	index: c_ulong,
}

impl Default for BaseHandle {
	fn default() -> Self {
		Self::INVALID
	}
}

/// Number of bits needed to represent max edicts.
pub const MAX_EDICT_BITS: u8 = 11;

/// Maximum number of edicts on a server.
pub const MAX_EDICTS: usize = 1 << MAX_EDICT_BITS;

impl BaseHandle {
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

vtable! {
	pub HandleEntityVt {
		pub fn destructor();
		#[cfg(not(windows))]
		pub fn destructor_2();
		pub fn set_ref_ehandle(handle: RefConst<BaseHandle>);
		pub fn get_ref_ehandle() -> RefConst<BaseHandle>;
	}
}

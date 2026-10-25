use ::core::ffi::{
	c_char, c_float, c_int,
};
use ::rse_cpp::class;
use ::rse_math::Vector;
use ::rse_utl::cppdef::CUtlHash;

use super::{
	entities::{
		CBaseEntity, edict_t,
	},
	string_t,
};

class! {
	pub CSaveRestoreData: pub CSaveRestoreSegment, pub CGameSaveRestoreInfo {
		pub is_async: bool;
	}
}

class! {
	pub CSaveRestoreSegment {
		pub base_data: *mut c_char;
		pub current_data: *mut c_char;
		pub size: c_int;
		pub buffer_size: c_int;

		pub token_count: c_int;
		pub tokens: *mut *mut c_char;
	}
}

class! {
	pub CGameSaveRestoreInfo {
		pub level_info: saverestorelevelinfo_t;
		pub model_space_offset: Vector;
		pub table_count: c_int;
		pub table: *mut entitytable_t;
		pub current_entity: *mut CBaseEntity;
		pub entity_to_index: CUtlHash<CHashElement>;
	}
}

class! {
	pub CHashElement {
		pub entity: *const CBaseEntity;
		pub index: c_int;
	}
}

#[repr(C)]
pub struct saverestorelevelinfo_t {
	pub connection_count: c_int,
	pub level_list: [levellist_t; MAX_LEVEL_CONNECTIONS],
	pub use_landmark: c_int,
	pub landmark_name: [c_char; 20],
	pub landmark_offset: Vector,
	pub time: c_float,
	pub current_map_name: [c_char; MAX_MAP_NAME_SAVE],
	pub map_version: c_int,
}

pub const MAX_LEVEL_CONNECTIONS: usize = 16;

#[repr(C)]
pub struct levellist_t {
	// DECLARE_SIMPLE_DATADESC();
	pub map_name: [c_char; MAX_MAP_NAME_SAVE],
	pub landmark_name: [c_char; 32],
	pub ent_landmark: *mut edict_t,
	pub landmark_origin: Vector,
}

pub const MAX_MAP_NAME_SAVE: usize = 32;

// TODO: `entitytable_t`.
#[repr(C)]
pub struct entitytable_t<EHANDLE = EHandlePlaceholder_t /* CBaseEntity for server, C_BaseEntity for client */> {
	pub id: c_int,
	pub edict_index: c_int,
	pub save_entity_index: c_int,
	pub restore_entity_index: c_int,
	pub ent: EHANDLE,
	pub location: c_int,
	pub size: c_int,
	pub flags: c_int,
	pub class_name: string_t,
	pub global_name: string_t,
	pub landmark_model_space: Vector,
	pub model_name: string_t,

	// DECLARE_SIMPLE_DATADESC();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct EHandlePlaceholder_t {
	pub i: u32,
}

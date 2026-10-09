use ::core::ffi::{
	c_float, c_int, c_short, c_ushort,
};
use ::rse_cpp::class;
use ::rse_math::{
	Vector, cplane_t,
};

use super::{
	entities::CBaseEntity,
	csurface_t,
};

// TODO: `DISPSURF_FLAG_*` as a bitset.

class! {
	/// Structure that is returned when a box is swept through the world.
	#[derive(Debug)]
	pub CGameTrace: pub CBaseTrace {
		pub fraction_where_left_solid: c_float;
		pub surface: csurface_t;
		pub hitgroup: c_int;
		pub physics_bone: c_short;
		pub entity: *mut CBaseEntity;
		/// If `entity` is the world, then this is the static prop index.
		/// Otherwise, this is the hitbox index.
		pub hitbox: c_int;
	}
}

// `trace_t` is aliased to `CGameTrace`.
#[allow(non_camel_case_types)]
pub type trace_t = CGameTrace;

class! {
	/// Base structure for the result of a trace.
	#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
	pub CBaseTrace {
		pub start_pos: Vector;
		pub end_pos: Vector;
		pub plane: cplane_t;
		pub fraction: c_float;
		pub contents: c_int;
		pub disp_flags: c_ushort;
		pub all_solid: bool;
		pub started_in_solid: bool;
	}
}

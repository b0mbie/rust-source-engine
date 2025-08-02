use ::core::ffi::{
	c_float, c_int, c_short, c_ushort,
};

use super::{
	entities::BaseEntity,
	Vector, Plane, Surface,
};

// TODO: `DISPSURF_FLAG_*` as a bitset.

/// Structure that is returned when a box is swept through the world.
#[derive(Debug)]
#[repr(C)]
pub struct GameTrace {
	pub base: BaseTrace,
	pub fraction_where_left_solid: c_float,
	pub surface: Surface,
	pub hitgroup: c_int,
	pub physics_bone: c_short,
	pub entity: *mut BaseEntity,
	/// If `entity` is the world, then this is the static prop index.
	/// Otherwise, this is the hitbox index.
	pub hitbox: c_int,
}

// `trace_t` is aliased to `CGameTrace`.
pub type Trace = GameTrace;

/// Base structure for the result of a trace.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct BaseTrace {
	pub start_pos: Vector,
	pub end_pos: Vector,
	pub plane: Plane,
	pub fraction: c_float,
	pub contents: c_int,
	pub disp_flags: c_ushort,
	pub all_solid: bool,
	pub started_in_solid: bool,
}

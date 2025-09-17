// TODO: `wchar_t`.
#[allow(non_camel_case_types)]
pub type wchar_t = u16;

mod buffer;
pub use buffer::*;
mod keyvalues;
pub use keyvalues::*;
mod model;
pub use model::*;
mod sound_level;
pub use sound_level::*;
mod steam_id;
pub use steam_id::*;
mod string;
pub use string::*;
mod trace;
pub use trace::*;

pub mod cvar;
pub mod datatable;
pub mod entities;

/// Representation of the bounding volume of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SolidType {
	/// No solid model.
	None = 0,
	/// A BSP tree.
	Bsp = 1,
	/// An AABB.
	Bbox = 2,
	/// An OBB (not implemented in the engine).
	Obb = 3,
	/// An OBB, constrained so that it can only yaw.
	ObbYaw = 4,
	/// Representation defined with a test function in the entity.
	Custom = 5,
	/// Solid *VPhysics* object.
	VPhysics = 6,
}

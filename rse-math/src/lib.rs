#![no_std]

mod aligned;
pub use aligned::*;
mod color;
pub use color::*;
mod matrix;
pub use matrix::*;
mod plane;
pub use plane::*;
mod vector;
pub use vector::*;

/// Vector that represents three-dimensional extrinsic Tait-Bryan rotations following the right-hand rule,
/// offset from the cardinal Z axis.
pub type QAngle = Vector;

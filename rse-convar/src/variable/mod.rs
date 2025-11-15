//! APIs for interacting with Console Variables, or *ConVars*.

pub mod low;

#[cfg(feature = "macros")]
mod macros;

mod ext;
pub use ext::*;
mod get_value;
pub use get_value::*;
mod params;
pub use params::*;

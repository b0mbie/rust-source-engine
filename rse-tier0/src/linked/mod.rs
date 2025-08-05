pub mod cppdef;

mod macros;
mod spew;
pub use spew::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0;

pub mod cppdef;

mod macros;

mod errors;
mod spew;
pub use spew::*;

pub(crate) const STR_FORMAT: *const ::core::ffi::c_char = c"%.*s".as_ptr();

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0;

#[cfg(feature = "gcc")]
mod gcc;
#[cfg(feature = "gcc")]
pub use gcc::TypeInfo;

#[cfg(not(feature = "gcc"))]
compile_error!("`std::type_info` emulation not supported for the specified environment");

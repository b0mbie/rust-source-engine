use ::core::ffi::c_int;

#[derive(Debug)]
#[repr(C)]
pub struct UtlMemory<T> {
	pub memory: *mut T,
	pub allocation_count: c_int,
	pub grow_size: c_int,
}

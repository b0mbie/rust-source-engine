use crate::{
	ptr_compat::PointerFrom,
	VtObject
};

/// Safely converts a reference to [`VtObject<From>`] to a reference to `VtObject<To>`.
pub const fn convert_vt_ref<From, To>(vt: &VtObject<From>) -> &VtObject<To>
where
	To: PointerFrom<From>,
{
	unsafe { &*(vt as *const _ as *const _) }
}

/// Safely converts a mutable reference to [`VtObject<From>`] to a mutable reference to `VtObject<To>`.
pub const fn convert_vt_mut<From, To>(vt: &mut VtObject<From>) -> &mut VtObject<To>
where
	To: PointerFrom<From>,
{
	unsafe { &mut *(vt as *mut _ as *mut _) }
}

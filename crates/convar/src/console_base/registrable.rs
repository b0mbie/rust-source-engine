use ::core::ptr::NonNull;
use ::rse_cpp::ptr_compat::{
	PointerFrom, convert_ref, convert_mut,
};

use crate::cppdef::Registrable;

pub type RegistrableRef = NonNull<Registrable>;
pub type RegistrableMut = NonNull<Registrable>;

pub const fn registrable_ref<T>(t: &T) -> RegistrableRef
where
	Registrable: PointerFrom<T>,
{
	RegistrableRef::from_ref(convert_ref(t))
}

pub const fn registrable_mut<T>(t: &mut T) -> RegistrableMut
where
	Registrable: PointerFrom<T>,
{
	RegistrableMut::from_mut(convert_mut(t))
}

const _: () = {
	use rse_cpp::ptr_compat::PointerFrom;
	const fn assert_from_ptr_to<From, To>()
	where
		To: PointerFrom<From>,
	{}
	assert_from_ptr_to::<crate::cppdef::ConCommand, Registrable>()
};

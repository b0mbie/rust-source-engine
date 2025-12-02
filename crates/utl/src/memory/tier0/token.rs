use ::core::{
	any::type_name,
	fmt,
	marker::PhantomData,
};
use ::rse_tier0::can_be_allocated;

pub trait Tier0Allocatable: Sized {
	const TOKEN: Tier0AllocateToken<Self> = const {
		match Tier0AllocateToken::INSTANCE {
			Some(token) => token,
			None => panic!("the given type cannot be directly allocated with the `tier0` allocator"),
		}
	};
}
impl<T> Tier0Allocatable for T {}

/// Zero-sized proof that `T` can be allocated with `tier0`.
#[repr(transparent)]
pub struct Tier0AllocateToken<T>(PhantomData<fn(T)>);
impl<T> Tier0AllocateToken<T> {
	pub const INSTANCE: Option<Self> = if can_be_allocated::<T>() {
		Some(Self(PhantomData))
	} else {
		None
	};

	pub const fn get() -> Option<Self> {
		Self::INSTANCE
	}
}

impl<T> fmt::Debug for Tier0AllocateToken<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Tier0AllocateToken<")?;
		f.write_str(type_name::<T>())?;
		f.write_str(">")
	}
}

impl<T> Clone for Tier0AllocateToken<T> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T> Copy for Tier0AllocateToken<T> {}

impl<T> PartialEq for Tier0AllocateToken<T> {
	fn eq(&self, other: &Self) -> bool {
		let _ = other;
		true
	}
}
impl<T> Eq for Tier0AllocateToken<T> {}

#[test]
fn instance_is_correct() {
	assert_eq!(Tier0AllocateToken::<()>::INSTANCE, None);
	assert!(Tier0AllocateToken::<u8>::INSTANCE.is_some());
	assert!(Tier0AllocateToken::<usize>::INSTANCE.is_some());
	assert!(Tier0AllocateToken::<*mut ()>::INSTANCE.is_some());
}

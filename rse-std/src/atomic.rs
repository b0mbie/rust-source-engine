use ::core::sync::atomic::*;

pub trait AssocAtomic {
	type Atomic;
}

pub type Atomic<T> = <T as AssocAtomic>::Atomic;

macro_rules! impl_atomic {
	($target:ident $atomic:ident) => {
		impl AssocAtomic for $target {
			type Atomic = $atomic;
		}
	};
}

impl_atomic!(u8 AtomicU8);
impl_atomic!(u16 AtomicU16);
impl_atomic!(u32 AtomicU32);
impl_atomic!(i8 AtomicI8);
impl_atomic!(i16 AtomicI16);
impl_atomic!(i32 AtomicI32);

use crate::{
	ptr_compat::{
		PointerFrom,
		convert_ref, convert_mut,
	},
	VtObject, AsObject, VTablePtr,
};

/// Wrapper type for C++ classes with VTables,
/// enabling better compile-time formal verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct WithVTable<VTable, T> {
	pub vtable: VTablePtr<VTable>,
	pub data: T,
}

impl<VTable, T> WithVTable<VTable, T> {
	pub const fn new(vtable: VTablePtr<VTable>, data: T) -> Self {
		Self {
			vtable,
			data,
		}
	}

	/// Returns a reference to [`VtObject<VTable>`].
	pub const fn as_object(&self) -> &VtObject<VTable> {
		convert_ref(self)
	}

	/// Returns a mutable reference to [`VtObject<VTable>`].
	pub const fn as_mut_object(&mut self) -> &mut VtObject<VTable> {
		convert_mut(self)
	}
}

impl<VTable, T> AsObject<VTable> for WithVTable<VTable, T> {
	fn as_object(&self) -> &VtObject<VTable> {
		self.as_object()
	}
}

unsafe impl<VTable, T> PointerFrom<WithVTable<VTable, T>> for VtObject<VTable> {}

unsafe impl<VTable, T, VTable2, U> PointerFrom<WithVTable<VTable2, U>> for WithVTable<VTable, T>
where
	VTable: PointerFrom<VTable2>,
	T: PointerFrom<U>,
{}

use ::core::ptr::NonNull;

/// Converts a pointer to `T` to a pointer to `U`.
/// 
/// The pointer can be safely dereferenced.
pub const fn convert_non_null<T, U>(ptr: NonNull<T>) -> NonNull<U>
where
	U: PointerFrom<T>,
{
	ptr.cast()
}

/// Safely converts a reference to `T` to a reference to `U`.
pub const fn convert_ref<T, U>(t: &T) -> &U
where
	U: PointerFrom<T>,
{
	unsafe { &*(t as *const T as *const U) }
}

/// Safely converts a mutable reference to `T` to a mutable reference to `U`.
pub const fn convert_mut<T, U>(t: &mut T) -> &mut U
where
	U: PointerFrom<T>,
{
	unsafe { &mut *(t as *mut T as *mut U) }
}

/// Converts a pointer to `T` to a pointer to `U`.
/// 
/// The pointer can be safely dereferenced.
pub const fn convert_ptr<T, U>(ptr: *const T) -> *const U
where
	U: PointerFrom<T>,
{
	ptr as *const U
}

/// Converts a mutable pointer to `T` to a mutable pointer to `U`.
/// 
/// The pointer can be safely dereferenced.
pub const fn convert_mut_ptr<T, U>(ptr: *mut T) -> *mut U
where
	U: PointerFrom<T>,
{
	ptr as *mut U
}

/// Marker trait that indicates that pointers to the implementing type
/// can be freely created from pointers to `Src`.
/// 
/// Implementors of this trait are said to be *pointer-from-compatible* with `Src`.
/// However, this is a one-way relationship -
/// `Src` may not be pointer-from-compatible with the implementing type,
/// as is the case when the implementing type represents a C++ type
/// that inherits from the C++ type that `Src` represents.
/// 
/// Due to some limitations in Rust,
/// `PointerFrom<T> for T` is not implemented,
/// as it would cause conflicting implementations.
/// 
/// # Safety
/// The implementing type must, indeed, be pointer-from-compatible with `Src`.
/// 
/// The C standard essentially defines *compatibility* between types to not exist
/// unless explicitly stated otherwise in prose.
/// However, *as a guideline*,
/// one of the following being true typically qualifies a type to be pointer-from-compatible with `Src`:
/// - The implementing type is a `repr(transparent)` wrapper over `Src`.
/// - The implementing type is defined ***identically*** to `Src`.
/// - The implementing type is the first field of `Src`
///   (typically requiring that the implementing type is `repr(C)`).
/// - The implementing type has *some* of the *first* fields of `Src`, at the exact same offsets;
///   `size_of::<Self>() <= size_of::<Dest>()`.
/// 
/// Besides these, however, there are other cases where some type may be pointer-from-compatible with `Src`.
/// Notably, [`WithVTable<VTable, T>`](crate::WithVTable) is pointer-from-compatible with `WithVTable<VTable2, U>`
/// if both `VTable` and `Src` are *pointer-from-compatible* with `VTable2` and `U` respectively.
/// Translating to C++ semantics,
/// different C++ types *can* be pointer-from-compatible
/// *if* the implementing C++ type *inherits* from the `Src` C++ type.
/// 
/// # Examples
/// This trait can be freely implemented for `repr(transparent)` wrappers:
/// ```
/// # use rse_cpp::PointerFrom;
/// pub type ForeignType = u8;
/// 
/// #[repr(transparent)]
/// pub struct ForeignWrapper(ForeignType);
/// 
/// // SAFETY: `ForeignWrapper` is `repr(transparent)` over `ForeignType`, and so it has the exact same layout and ABI.
/// unsafe impl PointerFrom<ForeignWrapper> for ForeignType {}
/// ```
/// 
/// For C++ types, the following is typically valid:
/// ```
/// # use rse_cpp::{PointerFrom, vtable};
/// vtable! {
///     pub BaseVt {
///         pub fn do_thing();
///     }
/// }
/// 
/// vtable! {
///     pub InheritedVtBase {
///         pub fn do_inherited_thing();
///     }
/// }
/// 
/// #[repr(C)]
/// pub struct InheritedVt {
///     pub base: BaseVt,
///     pub inherited: InheritedVtBase,
/// }
/// 
/// // SAFETY: `InheritedVt`, which is `repr(C)`, inherits from `BaseVt`, having it as the first field.
/// unsafe impl PointerFrom<InheritedVt> for BaseVt {}
/// ```
pub unsafe trait PointerFrom<Src: ?Sized> {}

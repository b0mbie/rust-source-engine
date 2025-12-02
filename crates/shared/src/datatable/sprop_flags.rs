use crate::cppdef::datatable::RawSendPropFlags;

impl SendPropFlags {
	pub const fn from_ref(flags: &RawSendPropFlags) -> &Self {
		unsafe { &*(flags as *const _ as *const Self) }
	}

	pub const fn from_mut(flags: &mut RawSendPropFlags) -> &mut Self {
		unsafe { &mut *(flags as *mut _ as *mut Self) }
	}
}

::rse_cpp::bitflags! {
	#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[repr(transparent)]
	pub struct SendPropFlags: RawSendPropFlags {
		/// Unsigned integer data.
		const UNSIGNED = 0;
		/// Treat float/vector like a world coordinate.
		/// 
		/// Note that the bit count is ignored in this case.
		const COORD = 1;
		/// For floating point, don't scale into range, just take value as is.
		const NO_SCALE = 2;
		/// For floating point, limit high value to range minus one bit unit.
		const ROUND_DOWN = 3;
		/// For floating point, limit low value to range minus one bit unit.
		const ROUND_UP = 4;
		/// For vectors, treat like a normal.
		const NORMAL = 5;
		/// Mark as exclude prop (not exclud-*ed*, but it points at another prop to be excluded).
		const EXCLUDE = 6;
		/// Use XYZ/Exponent encoding for vectors.
		const XYZE = 7;
		/// Indicate that the property is inside an array,
		/// so it shouldn't be put flattened property list.
		/// Its array will point at it when it needs to.
		const INSIDE_ARRAY = 8;
		/// Set for datatable props using one of the default datatable proxies like
		/// `SendProxy_DataTableToDataTable` that always send the data to all clients.
		const PROXY_ALWAYS_YES = 9;
		/// Indicate that this is an often-changed field, moved to head of the sendtable so that it gets a small index.
		const CHANGES_OFTEN = 10;
		/// Unused flag.
		/// 
		/// **Original description:** Set automatically if `SPROP_VECTORELEM` is used.
		const IS_A_VECTOR_ELEM = 11;
		/// Flag that is set automatically
		/// if it's a datatable with an offset of `0` that doesn't change the pointer
		/// (i.e. for all automatically-chained base classes).
		/// 
		/// In this case, it can get rid of this `SendPropDataTable` altogether
		/// and spare the trouble of walking the hierarchy more than necessary.
		const COLLAPSIBLE = 12;
		/// Like [`Self::COORD`], but with special handling for multiplayer games.
		const COORD_MP = 13;
		/// Like [`Self::COORD`], but with special handling for multiplayer games,
		/// where the fractional component only gets a 3 bits instead of 5.
		const COORD_MP_LOW_PRECISION = 14;
		/// Like [`Self::COORD_MP`], but coordinates are rounded to integral boundaries.
		const COORD_MP_INTEGRAL = 15;
	}
}

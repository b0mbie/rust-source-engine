use ::core::ffi::c_int;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SendPropFlags(pub c_int);

// TODO: Use flag consts.

::rse_cpp::flag_consts! {
	for c_int:
	/// Unsigned integer data.
	UNSIGNED = 1 << 0;
	/// Treat float/vector like a world coordinate.
	/// 
	/// Note that the bit count is ignored in this case.
	COORD = 1 << 1;
	/// For floating point, don't scale into range, just take value as is.
	NO_SCALE = 1 << 2;
	/// For floating point, limit high value to range minus one bit unit.
	ROUND_DOWN = 1 << 3;
	/// For floating point, limit low value to range minus one bit unit.
	ROUND_UP = 1 << 4;
	/// For vectors, treat like a normal.
	NORMAL = 1 << 5;
	/// Mark as exclude prop (not exclud-*ed*, but it points at another prop to be excluded).
	EXCLUDE = 1 << 6;
	/// Use XYZ/Exponent encoding for vectors.
	XYZE = 1 << 7;
	/// Indicate that the property is inside an array,
	/// so it shouldn't be put flattened property list.
	/// Its array will point at it when it needs to.
	INSIDE_ARRAY = 1 << 8;
	/// Set for datatable props using one of the default datatable proxies like
	/// `SendProxy_DataTableToDataTable` that always send the data to all clients.
	PROXY_ALWAYS_YES = 1 << 9;
	/// Indicate that this is an often-changed field, moved to head of the sendtable so that it gets a small index.
	CHANGES_OFTEN = 1 << 10;
	/// Unused flag.
	/// 
	/// **Original description:** Set automatically if `SPROP_VECTORELEM` is used.
	IS_A_VECTOR_ELEM = 1 << 11;
	/// Flag that is set automatically
	/// if it's a datatable with an offset of `0` that doesn't change the pointer
	/// (i.e. for all automatically-chained base classes).
	/// 
	/// In this case, it can get rid of this `SendPropDataTable` altogether
	/// and spare the trouble of walking the hierarchy more than necessary.
	COLLAPSIBLE = 1 << 12;
	/// Like [`COORD`], but with special handling for multiplayer games.
	COORD_MP = 1 << 13;
	/// Like [`COORD`], but with special handling for multiplayer games,
	/// where the fractional component only gets a 3 bits instead of 5.
	COORD_MP_LOW_PRECISION = 1 << 14;
	/// Like [`COORD_MP`], but coordinates are rounded to integral boundaries.
	COORD_MP_INTEGRAL = 1 << 15;
	/// Use [`UNSIGNED`] if needed - it's more efficient.
	VARINT = NORMAL;
}

use super::RawSendPropFlags;

::rse_cpp::bit_flag_consts! {
	for RawSendPropFlags:
	pub UNSIGNED = 0;
	pub COORD = 1;
	pub NO_SCALE = 2;
	pub ROUND_DOWN = 3;
	pub ROUND_UP = 4;
	pub NORMAL = 5;
	pub EXCLUDE = 6;
	pub XYZE = 7;
	pub INSIDE_ARRAY = 8;
	pub PROXY_ALWAYS_YES = 9;
	pub CHANGES_OFTEN = 10;
	pub IS_A_VECTOR_ELEM = 11;
	pub COLLAPSIBLE = 12;
	pub COORD_MP = 13;
	pub COORD_MP_LOW_PRECISION = 14;
	pub COORD_MP_INTEGRAL = 15;
}

pub const VARINT: RawSendPropFlags = NORMAL;

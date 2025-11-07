use ::core::ffi::{
	c_char, c_int, c_uchar,
};

use super::Crc32;

pub const MAX_PLAYER_NAME_LENGTH: usize = 32;
pub const SIGNED_GUID_LEN: usize = 32;
pub const MAX_CUSTOM_FILES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct player_info_t {
	// DECLARE_BYTESWAP_DATADESC();
	pub name: [c_char; MAX_PLAYER_NAME_LENGTH],
	pub user_id: c_int,
	pub guid: [c_char; SIGNED_GUID_LEN + 1],
	pub friends_id: u32,
	pub friends_name: [c_char; MAX_PLAYER_NAME_LENGTH],
	pub fake_player: bool,
	pub is_hltv: bool,
	#[cfg(feature = "replay")]
	pub is_replay: bool,
	pub custom_files: [Crc32; MAX_CUSTOM_FILES],
	pub files_downloaded: c_uchar,
}

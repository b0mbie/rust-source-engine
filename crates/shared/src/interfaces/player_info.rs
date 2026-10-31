use ::core::ffi::{
	CStr, c_int,
};
use ::rse_cpp::{
	transparent_wrapper,
	virtual_call,
};
use ::rse_game_interfaces::{
	InterfaceOfFactory, GameServerFactory,
};
use ::rse_interface::{
	Interface, ptr_interface,
};
use ::rse_math::{
	Vector, QAngle,
};

use crate::{
	cppdef::{
		CGlobalVars,
		IPlayerInfoManager, IPlayerInfo, IPlayerInfo_V2,
		INTERFACEVERSION_PLAYERINFOMANAGER,
	},
	ServerEdict,
};

pub use crate::cppdef::CBotCmd;

ptr_interface! {
	pub struct OPlayerInfoManager(PlayerInfoManager);
	Pointee = IPlayerInfoManager;
}
unsafe impl Send for OPlayerInfoManager {}
unsafe impl Sync for OPlayerInfoManager {}
impl Interface for OPlayerInfoManager {
	const IDENTIFIER: &CStr = INTERFACEVERSION_PLAYERINFOMANAGER;
}
impl InterfaceOfFactory for OPlayerInfoManager {
	type Factory = GameServerFactory;
}

impl PlayerInfoManager {
	pub fn player_info_mut<'a>(&mut self, edict: &'a mut ServerEdict) -> Option<&'a mut PlayerInfo> {
		unsafe {
			let ptr = virtual_call!(self.as_mut_inner() => get_player_info(edict.as_mut_ptr()));
			Some(PlayerInfo::from_mut(ptr.as_mut()?))
		}
	}
	pub fn player_info<'a>(&self, edict: &'a ServerEdict) -> Option<&'a PlayerInfo> {
		unsafe {
			let ptr = virtual_call!(self.as_inner() => get_player_info(edict.as_ptr() as _));
			Some(PlayerInfo::from_ref(ptr.as_ref()?))
		}
	}
	pub fn global_vars(&self) -> *mut CGlobalVars {
		unsafe { virtual_call!(self.as_inner() => get_global_vars()) }
	}
}

transparent_wrapper! {
	pub struct PlayerInfo for IPlayerInfo as "IPlayerInfo";
}

impl PlayerInfo {
	/// Returns the UTF-8 encoded name of the player.
	pub fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_inner() => get_name())) }
	}
	/// Returns the user ID (slot number).
	pub fn user_id(&self) -> UserId {
		unsafe { virtual_call!(self.as_inner() => get_user_id()) }
	}
	/// Returns the string of the player's network (i.e. Steam) ID.
	pub fn network_id(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_inner() => get_network_id_string())) }
	}
	/// Returns the team the player is on.
	pub fn team_index(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_team_index()) }
	}
	/// Changes the player to a new team, if the game DLL logic allows it.
	pub fn change_team(&mut self, team_num: c_int) {
		unsafe { virtual_call!(self.as_mut_inner() => change_team(team_num)) }
	}
	/// Returns the number of kills this player has.
	/// 
	/// The exact meaning of the value is mod-dependent.
	pub fn frags(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_frag_count()) }
	}
	/// Returns the number of kills this player has.
	/// 
	/// The exact meaning of the value is mod-dependent.
	pub fn deaths(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_frag_count()) }
	}
	/// Returns `true` if the player slot is actually valid.
	pub fn is_connected(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_connected()) }
	}
	/// Returns the armor/health of the player.
	/// 
	/// The exact meaning of the value is mod-dependent.
	pub fn armor(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_armor_value()) }
	}
}

transparent_wrapper! {
	pub struct PlayerInfo2 for IPlayerInfo_V2 as "IPlayerInfo_V2";
}

impl PlayerInfo2 {
	/// Returns `true` if the player is an HLTV spectator.
	// TODO: Clarify what this means.
	pub fn is_hltv(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_hltv()) }
	}
	/// Returns `true` if the player is a player.
	pub fn is_player(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_player()) }
	}
	/// Returns `true` if the player is a fake client.
	pub fn is_fake_client(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_fake_client()) }
	}
	/// Returns `true` if the player is dead.
	pub fn is_dead(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_dead()) }
	}
	/// Returns `true` if the player is in a vehicle.
	pub fn is_in_a_vehicle(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_in_a_vehicle()) }
	}
	/// Returns `true` if the player is an observer.
	// TODO: Clarify what this means.
	pub fn is_observer(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_observer()) }
	}
	/// Returns `true` if the player is a replay spectator.
	// TODO: Clarify what this means.
	pub fn is_replay(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_replay()) }
	}

	/// Returns the player's absolute origin (position).
	pub fn abs_origin(&self) -> Vector {
		unsafe { virtual_call!(self.as_inner() => get_abs_origin()) }
	}
	/// Returns the player's absolute angles (orientation).
	pub fn abs_angles(&self) -> QAngle {
		unsafe { virtual_call!(self.as_inner() => get_abs_angles()) }
	}
	/// Returns the lowest corner of the player's collision bounding box.
	pub fn mins(&self) -> Vector {
		unsafe { virtual_call!(self.as_inner() => get_player_mins()) }
	}
	/// Returns the highest corner of the player's collision bounding box.
	pub fn maxs(&self) -> Vector {
		unsafe { virtual_call!(self.as_inner() => get_player_maxs()) }
	}

	/// Returns the name of the weapon currently being carried.
	pub fn weapon_name(&self) -> &CStr {
		unsafe {
			let ptr = virtual_call!(self.as_inner() => get_weapon_name());
			CStr::from_ptr(ptr)
		}
	}
	/// Returns the name of the player model in use.
	pub fn model_name(&self) -> &CStr {
		unsafe {
			let ptr = virtual_call!(self.as_inner() => get_model_name());
			CStr::from_ptr(ptr)
		}
	}
	
	/// Returns the current health of the player.
	pub fn health(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_health()) }
	}
	/// Returns the maximum health of the player.
	pub fn health_max(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_max_health()) }
	}

	/// Returns the last user input from this player.
	pub fn last_user_command(&self) -> CBotCmd {
		unsafe { virtual_call!(self.as_inner() => get_last_user_command()) }
	}
}

pub type UserId = c_int;

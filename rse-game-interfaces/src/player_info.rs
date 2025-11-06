use ::core::ffi::{
	CStr, c_int,
};
use ::rse_cpp::{
	AsObject, virtual_call, vt_object_wrapper, owned_vt_object_wrapper,
	VtObject, VtObjectWrapper,
};
use ::rse_shared::{
	cppdef::GlobalVars,
	ServerEdict,
};
use ::rse_interface::Interface;
use ::rse_math::{
	Vector, QAngle,
};

use crate::{
	cppdef::{
		PlayerInfoManagerVt, PlayerInfoVt, PlayerInfo2Vt,
		INTERFACEVERSION_PLAYERINFOMANAGER,
		BotManagerVt, BotControllerVt,
		INTERFACEVERSION_PLAYERBOTMANAGER,
	},
	InterfaceOfFactory, GameServerFactory,
};

pub use crate::cppdef::BotCmd;

pub trait PlayerInfoManagerImpl: AsObject<PlayerInfoManagerVt> {
	fn player_info_mut<'a>(&self, edict: &'a mut ServerEdict) -> Option<&'a mut PlayerInfo2> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => get_player_info(edict.as_mut_ptr()))?;
			Some(PlayerInfo2::from_object_mut(VtObject::from_ptr_mut(ptr)))
		}
	}
	fn player_info<'a>(&self, edict: &'a ServerEdict) -> Option<&'a PlayerInfo2> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => get_player_info(edict.as_ptr() as _))?;
			Some(PlayerInfo2::from_object_const(VtObject::from_ptr_const(ptr)))
		}
	}
	fn global_vars_mut(&self) -> &'static mut GlobalVars {
		unsafe { &mut *virtual_call!(self.as_object() => get_global_vars()) }
	}
	fn global_vars(&self) -> &'static GlobalVars {
		unsafe { &*virtual_call!(self.as_object() => get_global_vars()) }
	}
}
impl<T: ?Sized + AsObject<PlayerInfoManagerVt>> PlayerInfoManagerImpl for T {}

pub trait PlayerInfoImpl: AsObject<PlayerInfoVt> {
	/// Returns the UTF-8 encoded name of the player.
	fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => get_name())) }
	}
	/// Returns the user ID (slot number).
	fn user_id(&self) -> UserId {
		unsafe { virtual_call!(self.as_object() => get_user_id()) }
	}
	/// Returns the string of the player's network (i.e. Steam) ID.
	fn network_id(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => get_network_id_string())) }
	}
	/// Returns the team the player is on.
	fn team_index(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_team_index()) }
	}
	/// Changes the player to a new team, if the game DLL logic allows it.
	fn change_team(&self, team_num: c_int) {
		unsafe { virtual_call!(self.as_object() => change_team(team_num)) }
	}
	/// Returns the number of kills this player has.
	/// 
	/// The exact meaning of the value is mod-dependent.
	fn frags(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_frag_count()) }
	}
	/// Returns the number of kills this player has.
	/// 
	/// The exact meaning of the value is mod-dependent.
	fn deaths(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_frag_count()) }
	}
	/// Returns `true` if the player slot is actually valid.
	fn is_connected(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_connected()) }
	}
	/// Returns the armor/health of the player.
	/// 
	/// The exact meaning of the value is mod-dependent.
	fn armor(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_armor_value()) }
	}
}
impl<T: ?Sized + AsObject<PlayerInfoVt>> PlayerInfoImpl for T {}

macro_rules! v2_call {
	($object:expr => $field:ident($($arg:tt)*)) => {{
		let object = $object;
		(object.vtable().v2.$field)(object.as_ptr().cast(), $($arg)*)
	}};
}

pub trait PlayerInfo2Impl: AsObject<PlayerInfo2Vt> {
	/// Returns `true` if the player is an HLTV spectator.
	// TODO: Clarify what this means.
	fn is_hltv(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_hltv()) }
	}
	/// Returns `true` if the player is a player.
	fn is_player(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_player()) }
	}
	/// Returns `true` if the player is a fake client.
	fn is_fake_client(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_fake_client()) }
	}
	/// Returns `true` if the player is dead.
	fn is_dead(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_dead()) }
	}
	/// Returns `true` if the player is in a vehicle.
	fn is_in_a_vehicle(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_in_a_vehicle()) }
	}
	/// Returns `true` if the player is an observer.
	// TODO: Clarify what this means.
	fn is_observer(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_hltv()) }
	}
	/// Returns `true` if the player is a replay spectator.
	// TODO: Clarify what this means.
	fn is_replay(&self) -> bool {
		unsafe { v2_call!(self.as_object() => is_replay()) }
	}

	/// Returns the player's absolute origin (position).
	fn abs_origin(&self) -> Vector {
		unsafe { v2_call!(self.as_object() => get_abs_origin()) }
	}
	/// Returns the player's absolute angles (orientation).
	fn abs_angles(&self) -> QAngle {
		unsafe { v2_call!(self.as_object() => get_abs_angles()) }
	}
	/// Returns the lowest corner of the player's collision bounding box.
	fn mins(&self) -> Vector {
		unsafe { v2_call!(self.as_object() => get_abs_origin()) }
	}
	/// Returns the highest corner of the player's collision bounding box.
	fn maxs(&self) -> Vector {
		unsafe { v2_call!(self.as_object() => get_abs_origin()) }
	}

	/// Returns the name of the weapon currently being carried.
	fn weapon_name(&self) -> &CStr {
		unsafe { CStr::from_ptr(v2_call!(self.as_object() => get_weapon_name())) }
	}
	/// Returns the name of the player model in use.
	fn model_name(&self) -> &CStr {
		unsafe { CStr::from_ptr(v2_call!(self.as_object() => get_model_name())) }
	}
	
	/// Returns the current health of the player.
	fn health(&self) -> c_int {
		unsafe { v2_call!(self.as_object() => get_health()) }
	}
	/// Returns the maximum health of the player.
	fn health_max(&self) -> c_int {
		unsafe { v2_call!(self.as_object() => get_health()) }
	}

	/// Returns the last user input from this player.
	fn last_user_command(&self) -> BotCmd {
		unsafe { v2_call!(self.as_object() => get_last_user_command()) }
	}
}
impl<T: ?Sized + AsObject<PlayerInfo2Vt>> PlayerInfo2Impl for T {}

pub type UserId = c_int;

pub trait BotManagerImpl: AsObject<BotManagerVt> {
	// TODO: Is it possible to make it so that `create_bot` can be used with `bot_controller_for` with `mut`?
	fn bot_controller_for<'a>(&self, edict: &'a ServerEdict) -> Option<&'a BotController> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => get_bot_controller(edict.as_ptr() as _))?;
			Some(BotController::from_object_mut(VtObject::from_ptr_mut(ptr)))
		}
	}
	fn create_bot(&self, name: &CStr) -> Option<&ServerEdict> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => create_bot(name.as_ptr()));
			if !ptr.is_null() {
				Some(ServerEdict::from_c_edict(&*ptr))
			} else {
				None
			}
		}
	}
	fn bot(&self, name: &CStr) -> Option<&BotController> {
		let edict = self.create_bot(name)?;
		self.bot_controller_for(edict)
	}
}
impl<T: ?Sized + AsObject<BotManagerVt>> BotManagerImpl for T {}

pub trait BotControllerImpl: AsObject<BotControllerVt> {
	// TODO: `IBotController` methods.
}
impl<T: ?Sized + AsObject<BotControllerVt>> BotControllerImpl for T {}

owned_vt_object_wrapper! {
	pub struct PlayerInfoManager for PlayerInfoManagerVt;
}
unsafe impl Interface for PlayerInfoManager {
	const IDENTIFIER: &CStr = INTERFACEVERSION_PLAYERINFOMANAGER;
}
impl InterfaceOfFactory for PlayerInfoManager {
	type Factory = GameServerFactory;
}

vt_object_wrapper! {
	pub struct PlayerInfo2 for PlayerInfo2Vt;
}
impl AsObject<PlayerInfoVt> for PlayerInfo2 {
	fn as_object(&self) -> &VtObject<PlayerInfoVt> {
		unsafe { VtObject::from_ptr_const(self.object.as_ptr().cast()) }
	}
}

owned_vt_object_wrapper! {
	pub struct BotManager for BotManagerVt;
}
unsafe impl Interface for BotManager {
	const IDENTIFIER: &CStr = INTERFACEVERSION_PLAYERBOTMANAGER;
}
impl InterfaceOfFactory for BotManager {
	type Factory = GameServerFactory;
}

vt_object_wrapper! {
	pub struct BotController for BotControllerVt;
}

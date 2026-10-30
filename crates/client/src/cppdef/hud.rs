use ::core::ffi::{
	c_char, c_float, c_int,
};
use ::rse_cpp::class;
use ::rse_game_interfaces::cppdef::CGameEventListener;
use ::rse_math::Color;
use ::rse_utl::cppdef::{
	CUtlDict,
	CUtlMap,
	CUtlPriorityQueue,
	CUtlVector,
};

use super::GameActionSet_t;

class! {
	pub CHud {
		pub key_bits: c_int;
		#[cfg(not(feature = "xbox360"))]
		pub mouse_sensitivity: c_float;
		#[cfg(not(feature = "xbox360"))]
		pub mouse_sensitivity_factor: c_float;
		pub fov_sensitivity_adjust: c_float;

		pub clr_normal: Color;
		pub clr_caution: Color;
		pub clr_yellowish: Color;

		pub hud_list: CUtlVector<*mut CHudElement>;

		pub hud_textures_loaded: bool;

		pub icons: CUtlDict<*mut CHudTexture, c_int>;

		pub render_group_names: CUtlVector<*const c_char>;
		pub render_groups: CUtlMap<c_int, *mut CHudRenderGroup>;
		
		pub screen_shot_time: c_float;
	}
}

class! {
	pub CHudElement: pub virtual CGameEventListener {
		// DECLARE_CLASS_NOBASE( CHudElement );
		// pub virtual fn destructor();
		// #[cfg(not(windows))]
		// pub virtual fn destructor_2();
		pub virtual fn init();
		pub virtual fn vid_init();
		pub virtual fn level_init();
		pub virtual fn level_shutdown();
		pub virtual fn reset();
		pub virtual fn process_input();
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn should_draw() -> bool;
		pub virtual fn is_active() -> bool;
		pub virtual fn set_active(active: bool);
		pub virtual fn get_preferred_action_set() -> GameActionSet_t;
		pub virtual fn set_hidden_bits(bits: c_int);
		pub virtual fn get_render_group_priority() -> c_int;
		// pub virtual fn fire_game_event(event: *mut IGameEvent);
		pub active: bool;
		pub hidden_bits: c_int;
		pub element_name: *const c_char;
		pub needs_remove: bool;
		pub is_parented_to_client_dll_root_panel: bool;
		pub hud_render_groups: CUtlVector<c_int>;
	}
}

class! {
	pub CHudTexture {
		pub short_name: [c_char; 64];
		pub texture_file: [c_char; 64];

		pub render_using_font: bool;
		pub precached: bool;
		pub character_in_font: c_char;
		pub font: ::rse_vgui::cppdef::HFont;

		pub texture_id: c_int;
		pub tex_coords: [c_float; 4];

		pub rc: wrect_t;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct wrect_t {
	pub left: c_int,
	pub right: c_int,
	pub top: c_int,
	pub bottom: c_int,
}

class! {
	pub CHudRenderGroup {
		pub hidden: bool;
		pub locking_elements: *mut CUtlPriorityQueue<*mut CHudElement>;
	}
}

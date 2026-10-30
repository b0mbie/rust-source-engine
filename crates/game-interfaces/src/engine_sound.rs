use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::rse_cpp::{
	RefMut, virtual_call,
};
use ::rse_math::Vector;
use ::rse_shared::{
	cppdef::{
		soundlevel_t, CHAN_AUTO,
	},
	recipient_filter::RecipientFilter,
};
use ::rse_utl::Vector as UtlVector;

use crate::cppdef::{
	IEngineSound, SOUND_FROM_WORLD,
};

::rse_cpp::transparent_wrapper! {
	pub struct EngineSound for IEngineSound as "IEngineSound";
}

impl EngineSound {
	pub fn precache_sound(&mut self, sample: &CStr, preload: bool, is_ui_sound: bool) -> bool {
		unsafe { virtual_call!(mut self.as_mut_inner() => precache_sound(sample.as_ptr(), preload, is_ui_sound)) }
	}

	pub fn is_sound_precached(&self, sample: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_sound_precached(sample.as_ptr())) }
	}
	
	pub fn prefetch_sound(&mut self, sample: &CStr) {
		unsafe { virtual_call!(mut self.as_mut_inner() => prefetch_sound(sample.as_ptr())) }
	}

	pub fn sound_duration(&self, sample: &CStr) -> c_float {
		unsafe { virtual_call!(self.as_inner() => get_sound_duration(sample.as_ptr())) }
	}

	pub fn emit_sound_with_attenuation(&mut self, attenuation: c_float, params: EmitSound<'_>) {
		let EmitSound {
			filter, ent_index,
			channel, sample, volume,
			flags, pitch, special_dsp,
			origin, direction, origins,
			update_positions, sound_time,
			speaker_entity,
		} = params;
		unsafe {
			virtual_call!(mut self.as_mut_inner() => emit_sound_with_attenuation(
				RefMut::from_mut(filter.as_mut_inner()),
				ent_index, channel,
				sample.as_ptr(),
				volume, attenuation,
				flags, pitch, special_dsp,
				origin.map(|r| r as *const _).unwrap_or_default(),
				direction.map(|r| r as *const _).unwrap_or_default(),
				origins.map(|v| v.as_mut_ptr() as _).unwrap_or_default(),
				update_positions,
				sound_time,
				speaker_entity,
			))
		}
	}

	pub fn emit_sound_with_sound_level(&mut self, sound_level: soundlevel_t, params: EmitSound<'_>) {
		let EmitSound { filter, ent_index, channel, sample, volume, flags, pitch, special_dsp, origin, direction, origins, update_positions, sound_time, speaker_entity } = params;
		unsafe {
			virtual_call!(mut self.as_mut_inner() => emit_sound_with_sndlvl(
				RefMut::from_mut(filter.as_mut_inner()),
				ent_index, channel,
				sample.as_ptr(),
				volume, sound_level,
				flags, pitch, special_dsp,
				origin.map(|r| r as *const _).unwrap_or_default(),
				direction.map(|r| r as *const _).unwrap_or_default(),
				origins.map(|v| v.as_mut_ptr() as _).unwrap_or_default(),
				update_positions,
				sound_time,
				speaker_entity,
			))
		}
	}
}

pub struct EmitSound<'a> {
	pub filter: &'a mut RecipientFilter,
	pub ent_index: c_int,
	pub channel: c_int,
	pub sample: &'a CStr,
	pub volume: c_float,
	pub flags: c_int,
	pub pitch: c_int,
	pub special_dsp: c_int,
	pub origin: Option<&'a Vector>,
	pub direction: Option<&'a Vector>,
	pub origins: Option<&'a mut UtlVector<Vector>>,
	pub update_positions: bool,
	pub sound_time: c_float,
	pub speaker_entity: c_int,
}

impl<'a> EmitSound<'a> {
	pub const fn new(filter: &'a mut RecipientFilter, sample: &'a CStr,) -> Self {
		Self {
			filter,
			ent_index: SOUND_FROM_WORLD,
			channel: CHAN_AUTO,
			sample,
			volume: 1.0,
			flags: 0,
			pitch: 100,
			special_dsp: 0,
			origin: None,
			direction: None,
			origins: None,
			update_positions: true,
			sound_time: 0.0,
			speaker_entity: -1,
		}
	}
}

use ::core::ffi::{
	c_float, c_int,
};

#[allow(non_camel_case_types)]
pub type soundlevel_t = c_int;

macro_rules! presets {
	{
		$(
			$(#[$attr:meta])*
			$name:ident = $value:expr,
		)*
	} => {
		$(
			#[allow(non_upper_case_globals)]
			$(#[$attr])*
			pub const $name: soundlevel_t = $value;
		)*
	};
}

presets! {
	SNDLVL_NONE = 0,

	/// Rustling leaves.
	SNDLVL_20dB = 20,
	/// Whispering.
	SNDLVL_25dB = 25,
	/// Library.
	SNDLVL_30dB = 30,
	SNDLVL_35dB = 35,
	SNDLVL_40dB = 40,
	/// Refrigerator.
	SNDLVL_45dB = 45,

	/// Average home.
	// 3.9
	SNDLVL_50dB = 50,
	// 3.0
	SNDLVL_55dB = 55,

	SNDLVL_IDLE = 60,
	/// Normal conversation, clothes dryer.
	// 2.0
	SNDLVL_60dB = 60,

	/// Washing machine, dishwasher.
	// 1.5
	SNDLVL_65dB = 65,
	// 1.25
	SNDLVL_STATIC = 66,

	/// Car, vacuum cleaner, mixer, electric sewing machine.
	// 1.0
	SNDLVL_70dB = 70,

	SNDLVL_NORM = 75,
	/// Busy traffic.
	// 0.8
	SNDLVL_75dB = 75,

	/// Mini-bike, alarm clock, noisy restaurant, office tabulator, outboard motor, passing snowmobile.
	// 0.7
	SNDLVL_80dB = 80,
	// 0.7
	SNDLVL_TALKING = 80,
	/// Average factory, electric shaver.
	// 0.6
	SNDLVL_85dB = 85,
	/// Screaming child, passing motorcycle, convertible ride on frw.
	// 0.5
	SNDLVL_90dB = 90,
	SNDLVL_95dB = 95,
	/// Subway train, diesel truck, woodworking shop, pneumatic drill, boiler shop, jackhammer.
	// 0.4
	SNDLVL_100dB = 100,
	/// Helicopter, power mower.
	SNDLVL_105dB = 105,
	/// Snowmobile driver's seat, inboard motorboat, sandblasting.
	SNDLVL_110dB = 110,
	/// Auto horn, propeller aircraft.
	SNDLVL_120dB = 120,
	/// Air raid siren.
	SNDLVL_130dB = 130,

	/// Threshold of pain, gunshot, jet engine.
	// 0.27
	SNDLVL_GUNFIRE = 140,
	// 0.2
	SNDLVL_140dB = 140,

	// 0.2
	SNDLVL_150dB = 150,

	/// Rocket launching.
	SNDLVL_180dB = 180,
}

/// Used to encode 0-255 for regular `soundlevel_t`'s and 256-511 for goldsrc-compatible ones.
pub const MAX_SNDLVL_BITS: u8 = 9;
pub const MIN_SNDLVL_VALUE: soundlevel_t = 0;
pub const MAX_SNDLVL_VALUE: soundlevel_t = (1 << MAX_SNDLVL_BITS) - 1;

/// Limit for attenuation due to network encoding.
/// 
/// Attenuation is encoded as multiplied by `64` in an 8-bit integer value.
pub const MAX_ATTENUATION: c_float = 255.0 / 64.0;

pub const fn attn_to_sndlvl(a: c_float) -> soundlevel_t {
	if a >= 0.0 {
		50 + (20.0 / a) as soundlevel_t
	} else {
		0
	}
}
pub const fn sndlvl_to_attn(a: soundlevel_t) -> c_float {
	if a > 50 {
		20.0 / ((a - 50) as c_float)
	} else {
		4.0
	}
}

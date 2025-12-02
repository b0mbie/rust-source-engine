#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SoundLevel {
	None = 0,

	/// Rustling leaves.
	Db20 = 20,
	/// Whispering.
	Db25 = 25,
	/// Library.
	Db30 = 30,
	Db35 = 35,
	Db40 = 40,
	/// Refrigerator.
	Db45 = 45,

	/// Average home.
	// 3.9
	Db50 = 50,
	Db55 = 55,
	/// Normal conversation, clothes dryer.
	// 2.0
	#[doc(alias = "Idle")]
	Db60 = 60,

	/// Washing machine, dishwasher.
	// 1.5
	Db65 = 65,
	// 1.25
	Static = 66,

	/// Car, vacuum cleaner, mixer, electric sewing machine.
	// 1.0
	Db70 = 70,

	/// Busy traffic.
	// 0.8
	#[doc(alias = "Normal")]
	Db75 = 75,

	/// Mini-bike, alarm clock, noisy restaurant, office tabulator, outboard motor, passing snowmobile.
	// 0.7
	#[doc(alias = "Talking")]
	Db80 = 80,
	/// Average factory, electric shaver.
	// 0.6
	Db85 = 85,
	/// Screaming child, passing motorcycle, convertible ride on frw.
	// 0.5
	Db90 = 90,
	Db95 = 95,
	/// Subway train, diesel truck, woodworking shop, pneumatic drill, boiler shop, jackhammer.
	// 0.4
	Db100 = 100,
	/// Helicopter, power mower.
	Db105 = 105,
	/// Snowmobile driver's seat, inboard motorboat, sandblasting.
	Db110 = 110,
	/// Auto horn, propeller aircraft.
	Db120 = 120,
	/// Air raid siren.
	Db130 = 130,

	/// *Threshold of pain*, gunshot, jet engine.
	// 0.27
	#[doc(alias = "Gunfire")]
	Db140 = 140,

	// 0.2
	Db150 = 150,

	/// Rocket launching.
	Db180 = 180,
}

#[allow(non_upper_case_globals)]
impl SoundLevel {
	pub const Idle: Self = Self::Db60;
	pub const Normal: Self = Self::Db75;
	pub const Talking: Self = Self::Db80;
	pub const Gunfire: Self = Self::Db140;
}

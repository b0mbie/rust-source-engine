pub const MAX_STEAM_CONTROLLERS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SteamPadAxis {
	LeftPadAxisX,
	LeftPadAxisY,
	RightPadAxisX,
	RightPadAxisY,
	LeftTriggerAxis,
	RightTriggerAxis,
	GyroAxisPitch,
	GyroAxisRoll,
	GyroAxisYaw,
}

impl SteamPadAxis {
	pub const MAX: Self = Self::GyroAxisYaw;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SteamKey {
	Null,
	ButtonA,
	ButtonB,
	ButtonX,
	ButtonY,
	ButtonUp,
	ButtonRight,
	ButtonDown,
	ButtonLeft,
	ButtonLeftBumper,
	ButtonRightBumper,
	ButtonLeftTrrigger,
	ButtonRightTrrigger,
	ButtonLeftGrip,
	ButtonRightGrip,
	ButtonLPadTouch,
	ButtonRPadTouch,
	ButtonLPadClick,
	ButtonRPadClick,
	ButtonLPadUp,
	ButtonLPadRight,
	ButtonLPadDown,
	ButtonLPadLeft,
	ButtonRPadUp,
	ButtonRPadRight,
	ButtonRPadDown,
	ButtonRPadLeft,
	ButtonSelect,
	ButtonStart,
	ButtonSteam,
	ButtonInactiveStart,
	VirtBtnF1,						// These are "virtual" buttons. Useful if you want to have flow that maps an action to button code to be interpreted by some UI that accepts keystrokes, but you
	VirtBtnF2,						// don't want to map to real button (perhaps because it would be interpreted by UI in a way you don't like). 																																										
	VirtBtnF3,
	VirtBtnF4,
	VirtBtnF5,
	VirtBtnF6,
	VirtBtnF7,
	VirtBtnF8,
	VirtBtnF9,
	VirtBtnF10,
	VirtBtnF11,
	VirtBtnF12,
	MaxKeys,
}

mod steam_controller;
pub use steam_controller::*;

pub const MAX_JOYSTICKS: usize = 1;
pub const MOUSE_BUTTON_COUNT: usize = 5;

pub const JOYSTICK_MAX_BUTTON_COUNT: usize = 32;
pub const JOYSTICK_POV_BUTTON_COUNT: usize = 4;
pub const JOYSTICK_AXIS_BUTTON_COUNT: usize = JoystickAxis::Max as usize * 2;

pub const STEAMCONTROLLER_MAX_BUTTON_COUNT: usize = SteamKey::MaxKeys as usize - 1;
pub const STEAMCONTROLLER_AXIS_BUTTON_COUNT: usize = SteamPadAxis::MAX as usize * 2;

// On Linux, the order is XYZURV, but on other platforms it's XYZRUV.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JoystickAxis {
	X = 0,
	Y,
	Z, #[cfg(unix)] U,
	R, #[cfg(not(unix))] U,
	V,
	Max,
}

const fn joystick_button_internal(joystick: usize, button: usize) -> isize {
	(ButtonCode::JoystickFirstButton as usize + (joystick * JOYSTICK_MAX_BUTTON_COUNT) + button) as _
}
const fn joystick_pov_button_internal(joystick: usize, button: usize) -> isize {
	(ButtonCode::JoystickFirstPovButton as usize + (joystick * JOYSTICK_POV_BUTTON_COUNT) + button) as _
}
const fn joystick_axis_button_internal(joystick: usize, button: usize) -> isize {
	(ButtonCode::JoystickFirstAxisButton as usize + (joystick * JOYSTICK_AXIS_BUTTON_COUNT) + button) as _
}

const fn steamcontroller_button_internal(joystick: usize, button: usize) -> isize {
	(ButtonCode::SteamControllerFirstButton as usize + (joystick * STEAMCONTROLLER_MAX_BUTTON_COUNT) + button) as _
}
const fn steamcontroller_axis_button_internal(joystick: usize, button: usize) -> isize {
	(ButtonCode::STEAMCONTROLLER_FIRST_AXIS_BUTTON as usize + (joystick * STEAMCONTROLLER_AXIS_BUTTON_COUNT) + (button)) as _
}

#[allow(non_upper_case_globals)]
impl ButtonCode {
	pub const COUNT: usize = Self::ButtonCodeLast as usize - Self::KeyFirst as usize + 1;

	pub const KeyFirst: Self = Self::None;
	pub const KeyNone: Self = Self::KeyFirst;
	pub const KeyLast: Self = Self::KeyScrolllocktoggle as _;
	pub const KEY_COUNT: usize = Self::KeyLast as usize - Self::KeyFirst as usize + 1;

	pub const MouseFirst: Self = Self::MouseLeft;
	pub const MouseLast: Self = Self::MouseWheelDown;
	pub const MOUSE_COUNT: usize = Self::MouseLast as usize - Self::MouseFirst as usize + 1;

	#[cfg(not(feature = "xbox360"))]
	pub const NovintFirst: Self = Self::NovintLogo0;

	#[cfg(not(feature = "xbox360"))]
	pub const NovintLast: Self = Self::NovintPlus1;
	pub const JoystickFirst: Self = Self::JoystickFirstButton;
	pub const JoystickLast: Self = Self::JoystickLastAxisButton;

	#[cfg(not(feature = "xbox360"))]
	pub const SteamControllerFirst: Self = Self::SteamControllerFirstButton;

	pub const STEAMCONTROLLER_LAST_BUTTON: isize = steamcontroller_button_internal(
		MAX_STEAM_CONTROLLERS - 1, STEAMCONTROLLER_MAX_BUTTON_COUNT - 1,
	);
	pub const STEAMCONTROLLER_FIRST_AXIS_BUTTON: isize = Self::STEAMCONTROLLER_LAST_BUTTON + 1;
	pub const STEAMCONTROLLER_LAST_AXIS_BUTTON: isize = steamcontroller_axis_button_internal(
		MAX_STEAM_CONTROLLERS - 1, STEAMCONTROLLER_AXIS_BUTTON_COUNT - 1,
	);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum ButtonCode {
	Invalid = -1,
	None = 0,

	Key0,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	KeyA,
	KeyB,
	KeyC,
	KeyD,
	KeyE,
	KeyF,
	KeyG,
	KeyH,
	KeyI,
	KeyJ,
	KeyK,
	KeyL,
	KeyM,
	KeyN,
	KeyO,
	KeyP,
	KeyQ,
	KeyR,
	KeyS,
	KeyT,
	KeyU,
	KeyV,
	KeyW,
	KeyX,
	KeyY,
	KeyZ,
	KeyPad0,
	KeyPad1,
	KeyPad2,
	KeyPad3,
	KeyPad4,
	KeyPad5,
	KeyPad6,
	KeyPad7,
	KeyPad8,
	KeyPad9,
	KeyPadDivide,
	KeyPadMultiply,
	KeyPadMinus,
	KeyPadPlus,
	KeyPadEnter,
	KeyPadDecimal,
	KeyLbracket,
	KeyRbracket,
	KeySemicolon,
	KeyApostrophe,
	KeyBackquote,
	KeyComma,
	KeyPeriod,
	KeySlash,
	KeyBackslash,
	KeyMinus,
	KeyEqual,
	KeyEnter,
	KeySpace,
	KeyBackspace,
	KeyTab,
	KeyCapslock,
	KeyNumlock,
	KeyEscape,
	KeyScrolllock,
	KeyInsert,
	KeyDelete,
	KeyHome,
	KeyEnd,
	KeyPageup,
	KeyPagedown,
	KeyBreak,
	KeyLshift,
	KeyRshift,
	KeyLalt,
	KeyRalt,
	KeyLcontrol,
	KeyRcontrol,
	KeyLwin,
	KeyRwin,
	KeyApp,
	KeyUp,
	KeyLeft,
	KeyDown,
	KeyRight,
	KeyF1,
	KeyF2,
	KeyF3,
	KeyF4,
	KeyF5,
	KeyF6,
	KeyF7,
	KeyF8,
	KeyF9,
	KeyF10,
	KeyF11,
	KeyF12,
	KeyCapslocktoggle,
	KeyNumlocktoggle,
	KeyScrolllocktoggle,

	// Mouse
	MouseLeft = Self::KeyScrolllocktoggle as isize + 1,
	MouseRight,
	MouseMiddle,
	Mouse4,
	Mouse5,
	MouseWheelUp,
	MouseWheelDown,

	// Joystick
	JoystickFirstButton = Self::MouseWheelDown as isize + 1,
	JoystickLastButton = joystick_button_internal(MAX_JOYSTICKS - 1, JOYSTICK_MAX_BUTTON_COUNT - 1),
	JoystickFirstPovButton,
	JoystickLastPovButton = joystick_pov_button_internal(MAX_JOYSTICKS - 1, JOYSTICK_POV_BUTTON_COUNT - 1),
	JoystickFirstAxisButton,
	JoystickLastAxisButton = joystick_axis_button_internal(MAX_JOYSTICKS - 1, JOYSTICK_AXIS_BUTTON_COUNT - 1),
	
	#[cfg(not(feature = "xbox360"))]
	NovintLogo0 = Self::JoystickLastAxisButton as isize + 2,
	#[cfg(not(feature = "xbox360"))]
	NovintTriangle0,
	#[cfg(not(feature = "xbox360"))]
	NovintBolt0,
	#[cfg(not(feature = "xbox360"))]
	NovintPlus0,
	#[cfg(not(feature = "xbox360"))]
	NovintLogo1,
	#[cfg(not(feature = "xbox360"))]
	NovintTriangle1,
	#[cfg(not(feature = "xbox360"))]
	NovintBolt1,
	#[cfg(not(feature = "xbox360"))]
	NovintPlus1,

	// Steam Controller
	SteamControllerFirstButton = if cfg!(not(feature = "xbox360")) {
		Self::NovintPlus1 as isize + 1
	} else {
		Self::JoystickLastAxisButton as isize + 1
	},

	SteamControllerLast = Self::STEAMCONTROLLER_LAST_AXIS_BUTTON,
	ButtonCodeLast = Self::SteamControllerLast as isize + 1,

	// TODO: Helpers for Xbox360 and Steam Controller.
}

use ::rse_tier0::uintp;

pub mod controls;

mod border;
pub use border::*;
mod client_panel;
pub use client_panel::*;
mod engine_vgui;
pub use engine_vgui::*;
mod panel;
pub use panel::*;

pub type VPANEL = uintp;

pub type HScheme = u32;
pub type HTexture = u32;
pub type HCursor = u32;
pub type HPanel = u32;
pub const INVALID_PANEL: HPanel = 0xffffffff;
pub type HFont = u32;
pub const INVALID_FONT: HFont = 0;

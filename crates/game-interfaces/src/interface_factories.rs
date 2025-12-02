use ::core::ffi::CStr;
use ::rse_interface::{
	CreateInterfaceFn, FromRawInterface, RawInterface, RawInterfaceFactory,
	ReturnCode,
};

#[diagnostic::on_unimplemented(message = "`{Self}` is not a Source Engine interface")]
pub trait InterfaceOfFactory: Sized + FromRawInterface {
	type Factory;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AppSystemFactory(pub CreateInterfaceFn);
impl RawInterfaceFactory for AppSystemFactory {
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		unsafe { self.0.create_interface_raw(name, return_code) }
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct GameServerFactory(pub CreateInterfaceFn);
impl RawInterfaceFactory for GameServerFactory {
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		unsafe { self.0.create_interface_raw(name, return_code) }
	}
}

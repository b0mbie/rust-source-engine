use ::core::{
	ffi::CStr,
	marker::PhantomData,
};
use ::rse_interface::{
	CreateInterfaceFn, FromRawInterface, RawInterface, RawInterfaceFactory,
	InterfaceFactory, ReturnCode,
};

#[derive(Debug, Clone, Copy)]
pub struct InterfaceFactories<'a> {
	app_system_factory: CreateInterfaceFn,
	game_server_factory: CreateInterfaceFn,
	_life: PhantomData<fn(&'a ())>,
}

impl InterfaceFactories<'_> {
	pub const fn new(
		app_system_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn,
	) -> Self {
		Self {
			app_system_factory,
			game_server_factory,
			_life: PhantomData,
		}
	}

	pub fn create_interface<I: InterfaceOfFactory>(&self) -> Result<I, I::Error> {
		let factory = I::Factory::from_factories(self.app_system_factory, self.game_server_factory);
		factory.create_interface::<I>()
	}

	pub const fn app_system_factory(&self) -> CreateInterfaceFn {
		self.app_system_factory
	}

	pub const fn game_server_factory(&self) -> CreateInterfaceFn {
		self.game_server_factory
	}
}

#[diagnostic::on_unimplemented(message = "`{Self}` is not a Source Engine interface")]
pub trait InterfaceOfFactory: Sized + FromRawInterface {
	type Factory: Factory;
}

pub trait Factory: RawInterfaceFactory {
	fn from_factories(app_system_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> Self;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AppSystemFactory(CreateInterfaceFn);
impl RawInterfaceFactory for AppSystemFactory {
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		unsafe { self.0.create_interface_raw(name, return_code) }
	}
}
impl Factory for AppSystemFactory {
	fn from_factories(app_system_factory: CreateInterfaceFn, _: CreateInterfaceFn) -> Self {
		Self(app_system_factory)
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct GameServerFactory(CreateInterfaceFn);
impl RawInterfaceFactory for GameServerFactory {
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		unsafe { self.0.create_interface_raw(name, return_code) }
	}
}
impl Factory for GameServerFactory {
	fn from_factories(_: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> Self {
		Self(game_server_factory)
	}
}

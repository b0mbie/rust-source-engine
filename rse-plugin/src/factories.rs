use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::{
	CreateInterfaceFn, RawInterfaceFactory,
	InterfaceFactory,
};

#[derive(Debug, Clone, Copy)]
pub struct PluginFactories {
	app_system_factory: CreateInterfaceFn,
	game_server_factory: CreateInterfaceFn,
}

impl PluginFactories {
	pub const fn new(
		app_system_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn,
	) -> Self {
		Self {
			app_system_factory,
			game_server_factory,
		}
	}

	pub fn create_interface<I>(&self) -> Result<I, I::Error>
	where
		I: InterfaceOfFactory,
		I::Factory: Factory,
	{
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

pub trait Factory: RawInterfaceFactory {
	fn from_factories(app_system_factory: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> Self;
}
impl Factory for ::rse_game_interfaces::AppSystemFactory {
	fn from_factories(app_system_factory: CreateInterfaceFn, _: CreateInterfaceFn) -> Self {
		Self(app_system_factory)
	}
}
impl Factory for ::rse_game_interfaces::GameServerFactory {
	fn from_factories(_: CreateInterfaceFn, game_server_factory: CreateInterfaceFn) -> Self {
		Self(game_server_factory)
	}
}

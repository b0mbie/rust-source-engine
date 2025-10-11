use ::rse_convar::{
	console_base::CvarDllIdentifier,
	command::{
		low::ConCommandObject,
		con_command,
	},
	prelude::*,
};
use ::rse_plugin::{
	game_interfaces::cvar::{
		Cvar, CvarImpl,
	},
	prelude::*,
};
use ::rse_tier0::prelude::*;
use ::std::sync::{
	atomic::Ordering,
	RwLock, RwLockReadGuard, RwLockWriteGuard,
};

type AtomicDllIdentifier = ::atomic::Atomic<CvarDllIdentifier>;

static CVAR: RwLock<Option<Cvar>> = RwLock::new(None);
const POISON_EXPECT: &str = "Cvar interface shouldn't be poisoned";
fn cvar_read() -> RwLockReadGuard<'static, Option<Cvar>> {
	CVAR.read().expect(POISON_EXPECT)
}
fn cvar_write() -> RwLockWriteGuard<'static, Option<Cvar>> {
	CVAR.write().expect(POISON_EXPECT)
}

const FIRST_INIT_DLL_ID: CvarDllIdentifier = 0;
const UNINIT_DLL_ID: CvarDllIdentifier = FIRST_INIT_DLL_ID - 1;
static DLL_IDENTIFIER: AtomicDllIdentifier = AtomicDllIdentifier::new(UNINIT_DLL_ID);
fn dll_identifier() -> CvarDllIdentifier {
	DLL_IDENTIFIER.load(Ordering::Relaxed)
}
fn reset_dll_identifier() {
	unsafe { set_dll_identifier(UNINIT_DLL_ID) }
}
unsafe fn set_dll_identifier(dll_id: CvarDllIdentifier) {
	DLL_IDENTIFIER.store(dll_id, Ordering::Relaxed)
}

struct RseTestCmd;
impl Command for RseTestCmd {
	const NAME: &CStr = c"rse_test_cmd";
}
unsafe impl DllCommand for RseTestCmd {
	fn dll_identifier(&mut self) -> CvarDllIdentifier {
		dll_identifier()
	}
}
impl DispatchCommand for RseTestCmd {
	fn dispatch(&mut self, invocation: &Invocation) {
		con_msg!("{:?}", invocation.args());
	}
}

struct TestStatic {
	pub rse_test_cmd: ConCommandObject<'static, RseTestCmd>,
}

impl StaticPlugin for TestStatic {
	unsafe fn load(&mut self, factories: InterfaceFactories<'_>) -> bool {
		if cvar_read().is_none() {
			*cvar_write() = factories.create_interface::<Cvar>().ok();
		}
		if let Some(cvar) = cvar_write().as_mut() {
			unsafe {
				set_dll_identifier(cvar.allocate_dll_identifier());
				cvar.register(&mut self.rse_test_cmd);
			}
		}
		true
	}
	unsafe fn unload(&mut self) {
		let dll_id = dll_identifier();
		if let Some(cvar) = cvar_write().as_mut()
			&& dll_id >= FIRST_INIT_DLL_ID
		{
			unsafe { cvar.unregister_all(dll_id) }
			reset_dll_identifier();
		}
	}
}

impl Plugin for TestStatic {
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}

export_static_plugin_as! {
	PLUGIN: TestStatic = TestStatic {
		rse_test_cmd: con_command(RseTestCmd),
	};
}


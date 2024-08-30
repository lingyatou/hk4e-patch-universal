#![feature(str_from_utf16_endian)]

use std::{sync::RwLock, time::Duration};

use lazy_static::lazy_static;
use modules::Misc;
use windows::core::PCSTR;
use windows::Win32::System::Console;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetModuleHandleA};

mod interceptor;
mod marshal;
mod modules;
mod util;

use crate::modules::{Http, MhyContext, ModuleManager, Security};

unsafe fn thread_func() {
    let base = GetModuleHandleA(PCSTR::null()).unwrap().0 as usize;

    std::thread::sleep(Duration::from_secs(12));

    util::disable_memprotect_guard();
    Console::AllocConsole().unwrap();

    println!("Genshin Impact encryption patch\nMade by xeondev\nTo work with XilonenImpact: git.xeondev.com/reversedrooms/XilonenImpact");
    println!("Base: {:X}", base);

    let mut module_manager = MODULE_MANAGER.write().unwrap();
    module_manager.enable(MhyContext::<Http>::new(base));
    module_manager.enable(MhyContext::<Security>::new(base));
    module_manager.enable(MhyContext::<Misc>::new(base));

    println!("Successfully initialized!");
}

lazy_static! {
    static ref MODULE_MANAGER: RwLock<ModuleManager> = RwLock::new(ModuleManager::default());
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| thread_func());
    }

    true
}

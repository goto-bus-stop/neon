use libloading::Library;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

#[cfg(target_os = "windows")]
mod host_library {
    use libloading::os::windows::Library;
    use winapi::um::libloaderapi::GetModuleHandleA;
    pub fn get() -> libloading::Library {
        let host_handle = GetModuleHandleA(null_mut());
        Library::from_raw(host_handle).into()
    }
}

#[cfg(not(target_os = "windows"))]
mod host_library {
    use libloading::os::unix::Library;
    pub fn get() -> libloading::Library {
        Library::this().into()
    }
}

lazy_static::lazy_static! {
    static ref HOST_LIBRARY: Library = host_library::get();
}

pub fn from_host() -> NodeApi<'static> {
    NodeApi::new(&HOST_LIBRARY)
}

use super::raw::Local;
use std::os::raw::c_void;

pub unsafe extern "C" fn new(_isolate: *mut c_void, _this: Local, _callback: Local) -> *mut c_void { unimplemented!() }
pub unsafe extern "C" fn schedule(_thread_safe_cb: *mut c_void, _rust_callback: *mut c_void,
            _complete: unsafe extern fn(Local, Local, *mut c_void)) { unimplemented!() }
pub unsafe extern "C" fn delete(_thread_safe_cb: *mut c_void) { unimplemented!() }

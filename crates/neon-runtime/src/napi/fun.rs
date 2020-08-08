//! Facilities for working with JS functions.

use super::call::CCallback;
use super::raw::{Env, Local};
use std::os::raw::c_void;
use std::ptr::null;

use napi_dynamic_sys as napi;

/// Mutates the `out` argument provided to refer to a newly created `v8::Function`. Returns
/// `false` if the value couldn't be created.
pub unsafe extern "C" fn new(out: &mut Local, env: Env, callback: CCallback) -> bool {
    let status = (super::NAPI.napi_create_function)(
        env,
        null(),
        0,
        Some(std::mem::transmute(callback.static_callback)),
        callback.dynamic_callback,
        out as *mut Local,
    );

    status == napi::napi_status::napi_ok
}

pub unsafe extern "C" fn new_template(_out: &mut Local, _env: Env, _callback: CCallback) -> bool {
    unimplemented!()
}

pub unsafe extern "C" fn get_dynamic_callback(_env: Env, data: *mut c_void) -> *mut c_void {
    data
}

pub unsafe extern "C" fn call(out: &mut Local, env: Env, fun: Local, this: Local, argc: i32, argv: *mut c_void) -> bool {
    let status = (super::NAPI.napi_call_function)(env, this, fun, argc as usize, argv as *const _, out as *mut _);

    status == napi::napi_status::napi_ok
}

pub unsafe extern "C" fn construct(out: &mut Local, env: Env, fun: Local, argc: i32, argv: *mut c_void) -> bool {
    let status = (super::NAPI.napi_new_instance)(env, fun, argc as usize, argv as *const _, out as *mut _);

    status == napi::napi_status::napi_ok
}

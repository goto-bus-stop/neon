use napi_dynamic_sys as napi;

use super::raw::{Env, Local};

/// This API is currently unused, see https://github.com/neon-bindings/neon/issues/572
pub unsafe extern "C" fn to_object(out: &mut Local, env: Env, value: Local) -> bool {
    let status = (super::NAPI.napi_coerce_to_object)(env, value, out as *mut _);

    status == napi::napi_status::napi_ok
}

pub unsafe extern "C" fn to_string(out: &mut Local, env: Env, value: Local) -> bool {
    let status = (super::NAPI.napi_coerce_to_string)(env, value, out as *mut _);

    status == napi::napi_status::napi_ok
}

//! Facilities for working with Array `napi_value`s.

use super::raw::{Env, Local};

use napi_dynamic_sys as napi;

pub unsafe extern "C" fn new(out: &mut Local, env: Env, length: u32) {
    assert_eq!(
        napi!(napi_create_array_with_length(env, length as usize, out as *mut _)),
        napi::napi_status::napi_ok,
    );
}

/// Gets the length of a `napi_value` containing a JavaScript Array.
///
/// # Panics
/// This function panics if `array` is not an Array, or if a previous n-api call caused a pending
/// exception.
pub unsafe extern "C" fn len(env: Env, array: Local) -> u32 {
    let mut len = 0;
    assert_eq!(
        napi!(napi_get_array_length(env, array, &mut len as *mut _)),
        napi::napi_status::napi_ok
    );
    len
}

use std::mem::MaybeUninit;
use napi_dynamic_sys::NodeApi;

pub mod array;
pub mod arraybuffer;
pub mod buffer;
pub mod call;
pub mod class;
pub mod convert;
pub mod error;
pub mod fun;
pub mod mem;
pub mod object;
pub mod primitive;
pub mod raw;
pub mod scope;
pub mod string;
pub mod tag;
pub mod task;
pub mod handler;

static mut NAPI: MaybeUninit<NodeApi<'static>> = MaybeUninit::uninit();

#[inline(always)]
unsafe fn napi() -> &'static NodeApi<'static> {
    // assume_init() takes ownership, so we cannot do that
    // get_ref() would do what we want but is unstable
    // https://github.com/rust-lang/rust/issuse/63568
    &*NAPI.as_ptr()
}

pub unsafe fn initialize() {
    *NAPI.as_mut_ptr() = napi_dynamic_sys::from_host();
}

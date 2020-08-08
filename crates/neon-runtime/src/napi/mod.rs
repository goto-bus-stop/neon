use napi_dynamic_sys as napi;

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

lazy_static::lazy_static! {
    static ref NAPI: napi::NodeApi<'static> = napi::from_host();
}

#[macro_use]
pub(crate) mod bindings;

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

/// Initialize the N-API runtime.
///
/// This must be called before any other neon-runtime functions are used.
pub unsafe fn initialize() {
    bindings::load();
}

#[cfg(all(
    any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        target_arch = "asmjs"
    ),
    all(feature = "stdweb", not(feature = "wasm-bindgen"))
))]
#[macro_use]
extern crate stdweb;

#[cfg(not(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
)))]
pub use crate::native::Instant;

#[cfg(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
))]
pub use crate::wasm::Instant;

#[cfg(not(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
)))]
#[cfg(feature = "now")]
pub use crate::native::now;

#[cfg(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
))]
#[cfg(feature = "now")]
pub use crate::wasm::now;

pub use std::time::Duration;

#[cfg(not(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
)))]
mod native;
#[cfg(any(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    target_arch = "asmjs"
))]
mod wasm;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    all(feature = "stdweb", not(feature = "wasm-bindgen"))
))]
#[macro_use]
extern crate stdweb;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
pub use crate::native::Instant;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub use crate::wasm::Instant;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
#[cfg(feature = "now")]
pub use crate::native::now;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[cfg(feature = "now")]
pub use crate::wasm::now;

pub use std::time::Duration;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
mod native;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
mod wasm;

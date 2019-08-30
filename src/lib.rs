#[cfg(all(any(target_arch = "wasm32", target_arch = "asmjs"), feature = "stdweb"))]
#[macro_use]
extern crate stdweb;

#[cfg(any(
    not(any(target_arch = "wasm32", target_arch = "asmjs")),
    not(any(feature = "stdweb", feature = "wasm-bindgen"))
))]
pub use crate::native::Instant;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    any(feature = "stdweb", feature = "wasm-bindgen")
))]
pub use crate::wasm::Instant;

#[cfg(any(
    not(any(target_arch = "wasm32", target_arch = "asmjs")),
    not(any(feature = "stdweb", feature = "wasm-bindgen"))
))]
#[cfg(feature = "now")]
pub use crate::native::now;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    any(feature = "stdweb", feature = "wasm-bindgen")
))]
#[cfg(feature = "now")]
pub use crate::wasm::now;

pub use std::time::Duration;

#[cfg(any(
    not(any(target_arch = "wasm32", target_arch = "asmjs")),
    not(any(feature = "stdweb", feature = "wasm-bindgen"))
))]
mod native;
#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    any(feature = "stdweb", feature = "wasm-bindgen")
))]
mod wasm;

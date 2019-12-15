cfg_if::cfg_if! {
    if #[cfg(any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        target_arch = "asmjs"
    ))] {
        #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
        #[macro_use]
        extern crate stdweb;

        mod wasm;
        pub use wasm::Instant;
        #[cfg(feature = "now")]
        pub use crate::wasm::now;
    } else {
        mod native;
        pub use native::Instant;
        #[cfg(feature = "now")]
        pub use native::now;
    }
}

#[cfg(any(
    not(any(target_arch = "wasm32", target_arch = "asmjs")),
    not(any(feature = "stdweb", feature = "wasm-bindgen"))
))]
#[cfg(feature = "now")]
pub use crate::native::SystemTime;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    any(feature = "stdweb", feature = "wasm-bindgen")
))]
#[cfg(feature = "now")]
pub use crate::wasm::SystemTime;

pub use std::time::Duration;

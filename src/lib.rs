#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod instant;
        pub use crate::instant::Instant;
    } else if #[cfg(any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        target_arch = "asmjs"
    ))] {
        #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
        #[macro_use]
        extern crate stdweb;

        mod instant;
        pub use crate::instant::Instant;
        mod wasm;
        pub use crate::wasm::now;
        pub use wasm::SystemTime;
    } else {
        mod native;
        pub use native::Instant;
        pub use native::now;
        pub use native::SystemTime;
    }
}

pub use core::time::Duration;

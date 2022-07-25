# v0.1.13

## Added
- Add enabled by default `std` feature which can be disabled to use this crate in `#![no_std]`  crates

# v0.1.12
## Added 
- Add `SystemTime` which works in both native and WASM environments.

## Modified
- The `now` function is always available now: there is no need to enable the `now` feature any more. The `now` feature
  still exists (but doesn’t do anything) for backwards compatibility.
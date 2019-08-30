extern crate wasm_bindgen_test;

use instant::Instant;
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
// run these tests using: wasm-pack test --chrome --headless -- --features wasm-bindgen

#[wasm_bindgen_test]
fn test_instant_now() {
    let now = Instant::now();
    assert!(now.elapsed().as_nanos() > 0);
}

#[wasm_bindgen_test]
fn test_duration() {
    let now = Instant::now();
    let one_sec = Duration::from_secs(1);
    assert!(now.elapsed() < one_sec);
}

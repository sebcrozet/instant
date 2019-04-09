# Instant

I you call `std::time::Instant::now()` on a WASM platform, it will panic. This creates provides a partial
replacement for `std::time::Instant` that works on WASM too. This defines the type `instant::Instant` which is:

* A struct emulating the behavior of **std::time::Instant** if your are targeting `wasm32-unknown-unknown` or `wasm32-unknown-asmjs`
**and** you enabled either the `stdweb` or the `wasm-bindgen` feature. This emulation is based on the javascript `performance.now()` function.
* A type alias for `std::time::Instant` otherwise.



Note that even if the **stdweb** or **wasm-bindgen** feature is enabled, this crate will continue to rely on `std::time::Instant`
as long as you are not targeting wasm32. This allows for portable code that will work on both native and WASM platforms.

### The `now` feature.
By enabling the feature `now` the function `instant::now()` will be exported and will either:

* Call `time::precise_time_s() * 1000.0` when compiling for a non-WASM platform.
* Call `performance.now()` when compiling for a WASM platform with the features **stdweb** or **wasm-bindgen** enabled.

The result is expressed in milliseconds.

### Examples
#### Using `instant` for a native platform.
```toml
[dependencies]
instant = "0.1"
```

```rust
fn main() {
    // Will be the same as `std::time::Instant`.
    let now = instant::Instant::new();
}
```


#### Using `instant` for a WASM platform.
This examples shows the use of the `stdweb` feature. It would be similar with `wasm-bindgen`.
```toml
[dependencies]
instant = { version = "0.1", features = [ "stdweb" ] }
```

```rust
fn main() {
    // Will emulate `std::time::Instant` based on `performance.now()`.
    let now = instant::Instant::new();
}
```

#### Using `instant` for a any platform by enable a feature transitively.
```toml
[features]
stdweb = [ "instant/stdweb" ]
wasm-bindgen = [ "instant/wasm-bindgen" ]

[dependencies]
instant = "0.1"
```

```rust
fn main() {
    // Will select the proper implementation depending on the
    // feature selected by the user.
    let now = instant::Instant::new();
}
```


#### Using the `now` feature.
```toml
[features]
stdweb = [ "instant/stdweb" ]
wasm-bindgen = [ "instant/wasm-bindgen" ]

[dependencies]
instant = { version = "0.1", features = [ "now" ] }
```

```rust
fn main() {
    // Will select the proper implementation depending on the
    // feature selected by the user.
    let now_instant = instant::Instant::new();
    let now_milliseconds = instant::now(); // In milliseconds.
}
```

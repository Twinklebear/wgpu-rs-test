# wgpu-test

Trying out Rust + WebGPU for cross-platform native/web graphics development.

## Building for Web

Build with cargo and generate bindings with wasm-bindgen:

```
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir target/generated/ --web target/wasm32-unknown-unknown/debug/wgpu-test.wasm
```

Then run a web server in the repo root directory.
```
python -m http.server
```

At the moment this crashes since the web backend for wgpu-rs doesn't
implement some of the new APIs in the WebGPU spec.


# Rust crate layout

The Rust implementation and its JavaScript bindings are intentionally split by responsibility:

- `engine`: platform-neutral Rust implementation. Keep N-API, WebAssembly, and JavaScript types out of this crate.
- `node-binding`: native Node.js adapter built with `napi-rs` and published as `@turbo-napi/node`.
- `wasm-binding`: browser WebAssembly adapter built with `wasm-bindgen` for `wasm32-unknown-unknown` and published as `@turbo-napi/wasm`.

Add business behavior to `engine`, then expose a thin wrapper from each adapter. Wrappers should only handle JavaScript naming, value conversion, errors, and runtime-specific behavior.

The Wasm wrappers use `cfg_attr` so their Rust functions remain active during normal host analysis while the `wasm_bindgen` macro is expanded only for browser Wasm builds.

## Native build

```sh
pnpm --filter @turbo-napi/node build
```

## WebAssembly build

Install the target and `wasm-pack` once:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Then build the browser package:

```sh
pnpm --filter @turbo-napi/wasm build
```

The generated package is written to `crates/wasm-binding/pkg`. Its default initializer must be awaited before calling exports:

```ts
import init, { plus100 } from '@turbo-napi/wasm'

await init()
console.log(plus100(23))
```

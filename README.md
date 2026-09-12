# Rust Node + WebAssembly monorepo template

This template keeps application logic in Rust and exposes it to both Node.js and browsers without coupling the shared implementation to either JavaScript ABI.

## Architecture

```text
crates/engine       Pure Rust logic
      │
      ├── bindings/node    napi-rs adapter → @turbo-napi/node
      └── bindings/web     wasm-bindgen adapter → @turbo-napi/web
                    │
              packages/api                → @turbo-napi/api
```

- `crates/engine` contains platform-neutral Rust and its unit tests.
- `bindings/node` exposes the engine through N-API.
- `bindings/web` exposes the engine through `wasm32-unknown-unknown`.
- `packages/api` provides one TypeScript interface and selects the binding through package export conditions.
- `examples/react-wasm` demonstrates browser consumption without importing a binding directly.

## Requirements

- Node.js and pnpm versions from the root `package.json`
- A stable Rust toolchain
- `wasm-pack`
- The `wasm32-unknown-unknown` Rust target

Install the Rust tooling once:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Install JavaScript dependencies:

```sh
pnpm install
```

## Build

Build the complete workspace:

```sh
pnpm build
```

Or build bindings independently:

```sh
pnpm build:node
pnpm build:web
```

Run the Rust tests:

```sh
pnpm test:rust
```

## Common API

Both Node.js and browser consumers use the same asynchronous factory. The factory is asynchronous because WebAssembly must be initialized before its exports can be called.

```ts
import { createMathApi } from '@turbo-napi/api'

const math = await createMathApi()

math.add(8, 2)
math.subtract(8, 2)
math.multiply(8, 2)
math.divide(8, 2)
```

Node.js resolves the `node` export condition to the native addon. Browser bundlers resolve `browser`, or fall back to the WebAssembly implementation through `default`.

Explicit environment entry points are also available:

```ts
import { createMathApi } from '@turbo-napi/api/node'
import { createMathApi } from '@turbo-napi/api/web'
```

## Adding shared behavior

1. Implement and test the operation in `crates/engine/src/lib.rs`.
2. Add a thin N-API wrapper in `bindings/node/src/lib.rs`.
3. Add a thin wasm-bindgen wrapper in `bindings/web/src/lib.rs`.
4. Add the operation to `MathApi` and both binding objects in `packages/api/src`.
5. Run `pnpm build` and `pnpm test:rust`.

Keep JavaScript value conversion and runtime-specific behavior in the adapters. The engine should remain usable as an ordinary Rust library.

## Rename for a new project

When using this repository as a template, replace:

- the `@turbo-napi/*` npm scope
- the `turbo-*` Cargo package prefix
- repository, author, and package metadata

The directory names describe architectural roles and normally do not need to change.

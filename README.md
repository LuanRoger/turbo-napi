# Rust Node + WebAssembly monorepo template

This template keeps application logic in Rust and exposes it to both Node.js and browsers without coupling the shared implementation to either JavaScript ABI.

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

Node.js and browser consumers use the same function interface:

```ts
import { add, divide, multiply, subtract } from "@turbo-napi/api";

add(8, 2);
subtract(8, 2);
multiply(8, 2);
divide(8, 2);
```

Node.js resolves the `node` export condition to the native addon. Browser bundlers resolve `browser`, or fall back to the WebAssembly implementation through `default`.

Explicit environment entry points are also available when an application should not depend on conditional export resolution:

For a Node.js application:

```ts
import { add } from "@turbo-napi/api/node";
```

For a browser application:

```ts
import { add } from "@turbo-napi/api/web";
```

## TypeScript configuration

Every TypeScript project extends a preset from `@packages/typescript-config`:

- `base.json` contains strict, environment-neutral correctness rules.
- `library.json` configures ESM libraries built by a bundler.
- `browser.json` adds DOM, Vite, and arbitrary-extension support.
- `react.json` extends the browser preset with React JSX.
- `node.json` configures NodeNext modules and Node.js globals.

New projects should extend the most specific preset and keep only project-local settings in their own `tsconfig.json`:

```json
{
  "extends": "@packages/typescript-config/node.json",
  "include": ["src"]
}
```

Add `@packages/typescript-config` as a `workspace:*` development dependency in every package that extends a preset.

## Adding shared behavior

1. Implement and test the operation in `crates/engine/src/lib.rs`.
2. Add a thin N-API wrapper in `bindings/node/src/lib.rs`.
3. Add a thin wasm-bindgen wrapper in `bindings/web/src/lib.rs`.
4. Re-export the operation from the Node and web entries in `packages/api/src`.
5. Run `pnpm build` and `pnpm test:rust`.

Keep JavaScript value conversion and runtime-specific behavior in the adapters. The engine should remain usable as an ordinary Rust library.

## Rename for a new project

When using this repository as a template, replace:

- the `@turbo-napi/*` npm scope
- the `turbo-*` Cargo package prefix
- repository, author, and package metadata

The directory names describe architectural roles and normally do not need to change.

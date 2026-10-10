# Country Lookup

A WebAssembly HTTP service built with Spin that demonstrates build-time pre-initialization using [Wizer](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wizer).

## Overview

During the build process, Wizer executes the initialization logic (`wizer_initialize`) to parse and load the country dataset (`data/countries.json`) into an in-memory lookup table (`BTreeMap`). The resulting heap state is snapshot directly into the final WebAssembly component (`country_lookup_init.wasm`).

Because the data is baked into the binary at build time, the component handles requests instantly and runs without any runtime dependencies (no external database, network calls, or filesystem access).

## Build & Run

Ensure you have Spin, the Rust `wasm32-wasip2` target, and `wasmtime` installed.

Compile and start the application with a single command:

```bash
spin up --build
```

Spin will execute `build.sh` (which compiles the Rust code and snapshots it with Wizer) and serve the HTTP application on `http://127.0.0.1:3000`.

## Example Usage

Look up a country by its two-letter ISO code:

```bash
curl -i http://localhost:3000/de
```

### Example Response

```http
HTTP/1.1 200 OK
content-type: application/json
content-length: 53

{"name":"Germany","code":"DE","phone_prefix":"+49"}
```

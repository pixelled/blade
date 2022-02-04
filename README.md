## WASM

### Build & Run

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_blade --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/blade.wasm
```

Then serve `wasm` directory to browser. i.e.

```sh
# cargo install basic-http-server
basic-http-server wasm
```

### Loading Assets
Move `assets` into the folder `wasm`

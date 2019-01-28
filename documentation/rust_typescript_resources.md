http://timryan.org/2019/01/22/exporting-serde-types-to-typescript.html

https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html

I tried out this library in branch `wasm_ts_gen` and found that it worked, but it didn't export the types from rust.
So it is a really good tool for quickly generating the body of TS types that correspond to the types accepted by and returned from the server.
But there will be a significant manual step in naming all of the generated types.

It is good for ensuring correctness, but not good enough to use in an automated fashion.
I'm also not sure how good it is at handing uuid and chrono types...

#### Instructions
Install the latest version of `wasm-bindgen` with `cargo install wasm-bindgen-cli -f`.
Compile your library with `cargo build --target wasm32-unknown-unknown --release`.
Go to the `target/` directory and navigate down to `library_name/wasm32-unknown-unknown/release`.
Run `wasm-bindgen library_name.wasm --out-dir test` and then open `test/library_name.d.ts`.
This is the output.


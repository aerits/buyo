cargo build --bin client --target wasm32-unknown-unknown
mkdir server/static
cp -r assets server/static/
wasm-bindgen target/wasm32-unknown-unknown/debug/client.wasm --out-dir server/static --target web

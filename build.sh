cargo build --bin client --target wasm32-unknown-unknown
mkdir rserver/static
cp -r assets rserver/static/
cp rserver/index.html rserver/static/
wasm-bindgen target/wasm32-unknown-unknown/debug/client.wasm --out-dir rserver/static --target web

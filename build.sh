cargo build --bin client --target wasm32-unknown-unknown
mkdir client_app_output/static
cp -r assets client_app_output/static/
wasm-bindgen target/wasm32-unknown-unknown/debug/client.wasm --out-dir client_app_output/static --target web
